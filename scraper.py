import logging
import pathlib
import requests
from bs4 import BeautifulSoup
import pandas as pd
import numpy as np
import requests_cache
import pathvalidate
from data_structures import (
    ProgramStub,
    ProgramKind,
    DegreeCourse,
    GenEdCourse,
    GenEds,
    ElectiveCourse,
    ElectiveStub,
    GenEdStub,
    GENERIC_ELECTIVE_NAMES,
    DefaultGenEdCodes,
)

# Configure logging
logging.basicConfig(
    level=logging.WARNING, format="%(asctime)s - %(levelname)s - %(message)s"
)
logger = logging.getLogger(__name__)

# Enable caching
requests_cache.install_cache(
    "schedulebot_cache", expire_after=86400
)  # Cache expires after 1 day

EXCEL_DEBUGGING = True  # Trims sheetnames to 31

SITE_URL = "https://coursecatalog.benedictine.edu"


def trim_titles(s: str) -> str:
    """Trim titles to 31 characters for Excel compatibility."""
    if EXCEL_DEBUGGING and len(s) > 31:
        return s[:31]
    return s


def fetch_and_parse_url(url):
    """Fetch a URL and parse it with BeautifulSoup."""
    response = requests.get(url, timeout=10)
    response.raise_for_status()
    soup = BeautifulSoup(response.text, "html.parser")
    if response.from_cache:
        logger.info("Response for %s was served from cache.", url)
    else:
        logger.info("Response for %s was fetched from the server.", url)
    return soup


def extract_program_data(row, base_url):
    """Extract program data from a table row."""
    data = {}
    for i, td in enumerate(row.find_all("td")):
        spans = td.find_all("span")
        if len(spans) >= 2:
            key = spans[0].get_text(strip=True)
            value = spans[1].get_text(strip=True)
            data[key] = value
            if i == 0:  # Extract URL from the Program column
                a_tag = spans[1].find("a")
                if a_tag and a_tag.has_attr("href"):
                    data["URL"] = a_tag["href"]
    if data and data["Level"] == "UG":  # Filter out Master's programs
        specialization = data["Degree Type"] == "Bachelor with Specialization"
        if specialization:
            data["Degree Type"] = "Bachelor"
        return ProgramStub(
            data["Program"],
            ProgramKind(data["Degree Type"]),
            requests.compat.urljoin(base_url, data["URL"]) + "#suggestedsequencestext",
            specialization,
        )
    return None


def scrape_program_info(url):
    """Scrape program information from the given URL."""
    soup = fetch_and_parse_url(url)
    programs_section = soup.find("div", id="programstextcontainer").find("tbody")
    if not programs_section:
        raise ValueError("Could not find the programs section on the page.")

    programs = []
    for row in programs_section.find_all("tr"):
        program = extract_program_data(row, url)
        if program:
            programs.append(program)
    return programs


def process_semester_courses(
    semesters, current_year, current_semester, courses, prog_name
):
    """Process and save semester courses into the semesters dictionary."""
    if current_year in [
        "Freshman Year",
        "Sophomore Year",
        "Junior Year",
        "Senior Year",
    ]:
        semesters[f"{current_year}: {current_semester}"] = pd.Series(courses)
    else:
        logger.warning(
            "Skipping non-standard year: %s for program %s",
            current_year,
            prog_name,
        )


def scrape_bachelors_courses(prog: ProgramStub):
    """Scrape bachelor's courses for a given program."""
    soup = fetch_and_parse_url(prog.url)
    courses_section = soup.find("div", id="suggestedsequencestextcontainer").find(
        "table"
    )
    if not courses_section:
        raise ValueError(f"Could not find courses section for {prog.name}.")

    second_natural_world = False

    semesters = {}
    current_semester, current_year, courses = None, None, []

    for tr in courses_section.find_all("tr"):
        if "plangridterm" in tr.get("class", []):  # Semester row
            if current_semester and current_year and courses:
                process_semester_courses(
                    semesters, current_year, current_semester, courses, prog.name
                )
                courses = []
            current_semester = tr.find("th").get_text(strip=True)
        elif "plangridyear" in tr.get("class", []):  # Year row
            if current_semester and current_year and courses:
                process_semester_courses(
                    semesters, current_year, current_semester, courses, prog.name
                )
                courses = []
            current_year = tr.find("th", class_="year").get_text(strip=True)
        elif "plangridsum" not in tr.get("class", []) and "plangridtotal" not in tr.get(
            "class", []
        ):  # Course row
            tds = tr.find_all("td")
            if len(tds) == 3:
                code = tds[0].get_text(strip=True)
                title = tds[1].get_text(strip=True)
                hours = tds[2].get_text(strip=True)
                if hours == "NULL":
                    hours = "0"
                elif not hours:
                    logger.warning(
                        "Skipping malformed course row: %s in %s", title, prog.name
                    )
                    continue
                hours = int(hours)
                url = tds[0].find("a")["href"]
                courses.append(DegreeCourse(title, code, hours, url))
            elif len(tds) == 2:
                title = tds[0].get_text(strip=True)
                if title in GENERIC_ELECTIVE_NAMES:
                    continue  # Skip generic electives
                hours = tds[1].get_text(strip=True)
                if hours == "NULL":
                    hours = "0"
                if "Natural World" in title:
                    if hours == "3-4":
                        hours = "3" if second_natural_world else "4"
                        logger.warning(
                            "Assuming variable Natural World is %s credits in %s",
                            hours,
                            prog.name,
                        )
                    second_natural_world = True
                if "Mathematical Reasoning" in title and hours == "3-4":
                    logger.warning(
                        "Assuming Math Reasoning is 3 credits in %s",
                        prog.name,
                    )
                    hours = "3"
                elif not hours:
                    logger.warning(
                        "Skipping malformed course row: %s in %s", title, prog.name
                    )
                    continue
                hours = int(hours)
                try:
                    gened = GenEds(title)
                except ValueError:
                    courses.append(ElectiveStub(title, hours))
                else:
                    courses.append(GenEdStub(title, gened, hours))

    if current_semester and current_year and courses:
        process_semester_courses(
            semesters, current_year, current_semester, courses, prog.name
        )

    return pd.DataFrame(semesters)


def split_course(course) -> pd.Series:
    """Split course string into components."""
    if pd.isna(course):
        return pd.Series(["", np.nan], index=["Course", "Credits"])
    return pd.Series([str(course), course.credit], index=["Course", "Credits"])


def to_debug_view(df):
    col_dict = {}
    for col in df:
        col_dict[col] = df[col].apply(split_course)
    new_df = pd.concat(col_dict, axis=1)
    return new_df


def extract_gened_course(courses_section, gened):
    for tr in courses_section.find_all("tr"):
        tds = tr.find_all("td")
        if len(tds) == 3:
            code = tds[0].get_text(strip=True)
            title = tds[1].get_text(strip=True)
            hours = tds[2].get_text(strip=True)
            if hours == "NULL":
                hours = "0"
            elif not hours:
                logger.warning(
                    "Skipping malformed course row: %s in %s", title, gened.name
                )
                continue
            hours = int(hours)
            url = tds[0].find("a")["href"]
            yield GenEdCourse(title, code, gened.value, hours, url)


def scrape_gened_courselists():
    gened_courses = {}
    for gened in GenEds:
        courses = []
        if gened.Url:
            # Construct the URL for the GenEd
            url = requests.compat.urljoin(SITE_URL, gened.Url)
            soup = fetch_and_parse_url(url)
            courses_section = soup.find("div", id="textcontainer").find("table")
            if not courses_section:
                raise ValueError(f"Could not find courses section for {gened.name}.")
            for i in extract_gened_course(courses_section, gened):
                courses.append(i)
        elif gened.name in [i.name for i in DefaultGenEdCodes]:
            # Use default codes if no URL is provided
            for code in DefaultGenEdCodes[gened.name].value:
                details = course_lookup(code)
                courses.append(
                    GenEdCourse(
                        details["title"],
                        details["code"],
                        gened,
                        details["credits"],
                        details["url"],
                    )
                )
        elif gened.name == "EXERCISE_FITNESS":
            url = requests.compat.urljoin(SITE_URL, "general-education/")
            soup = fetch_and_parse_url(url)
            courses_section = soup.find("div", id="textcontainer").find_all("table")[1]
            if not courses_section:
                raise ValueError(f"Could not find courses section for {gened.name}.")
            for i in extract_gened_course(courses_section, gened):
                courses.append(i)
        gened_courses[gened.name] = pd.Series(courses)

    return pd.DataFrame(gened_courses)


def course_lookup(code: str) -> dict[str]:
    url = requests.compat.urljoin(SITE_URL, "/search") + "?P=" + code.upper()
    soup = fetch_and_parse_url(url)
    course_section = soup.find("div", class_="courseblock")
    course_code = course_section.find("span", class_="detail-code").get_text(strip=True)
    title = course_section.find("span", class_="detail-title").get_text(strip=True)
    credits_raw = course_section.find("span", class_="detail-hours_html").get_text(
        strip=True
    )
    credits = credits_raw.strip("()").split()[0]  # Extract just the number

    return {"code": course_code, "title": title, "credits": int(credits), "url": url}


def main():
    url = requests.compat.urljoin(SITE_URL, "/courses-instruction") + "#programstext"
    try:
        geneds = scrape_gened_courselists()
        geneds.to_pickle(
            pathlib.Path("scraped_programs").joinpath("General_Education.pkl")
        )
        geneds.to_excel("scraped_geneds.xlsx")
        programs = scrape_program_info(url)
        logger.info("Programs and their links:")
        with pd.ExcelWriter("scraped_programs.xlsx") as writer:
            for prog in programs:
                if prog.kind == ProgramKind.Bachelor:
                    try:
                        df = scrape_bachelors_courses(prog)
                        df.to_pickle(
                            pathlib.Path("scraped_programs").joinpath(
                                pathvalidate.sanitize_filename(prog.name).replace(
                                    " ", "_"
                                )
                                + ".pkl"
                            )
                        )
                        to_debug_view(df).to_excel(
                            writer, sheet_name=trim_titles(prog.name)
                        )
                    except Exception as e:
                        logger.error("An error occurred: %s in %s", e, prog.name)
    except Exception as e:
        logger.error("An error occurred: %s", e)
        raise


if __name__ == "__main__":
    main()
