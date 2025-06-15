import dataclasses
import glob
import logging
import pathlib
import zipfile

import numpy as np
import pandas as pd
import pathvalidate
import requests
import requests_cache
from bs4 import BeautifulSoup
from lxml import etree

from data_structures import (
    GENERIC_ELECTIVE_NAMES,
    DefaultGenEdCodes,
    GenEds,
    ProgramKind,
    ProgramStub,
)
from xml_structures import Course, CourseKind

# Configure logging
logging.basicConfig(level=logging.WARNING, format="%(asctime)s - %(levelname)s - %(message)s")
logger = logging.getLogger(__name__)

# Enable caching
requests_cache.install_cache("schedulebot_cache", expire_after=86400)  # Cache expires after 1 day

EXCEL_DEBUGGING = True  # Trims sheetnames to 31

IGNORE_GENED_STUBS = False  # True  # Geneds automatically added in validation, so ignore stubs in scraping


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


def process_semester_courses(semesters, semester_idx, current_year, current_semester, courses, prog_name):
    """Process and save semester courses into the semesters dictionary."""
    if current_year in [
        "Freshman Year",
        "Sophomore Year",
        "Junior Year",
        "Senior Year",
    ]:
        semesters[f"semester-{semester_idx}"] = pd.DataFrame(courses)
        semester_idx += 1
        return semester_idx
    else:
        logger.warning(
            "Skipping non-standard year: %s for program %s",
            current_year,
            prog_name,
        )


def scrape_bachelors_courses(prog: ProgramStub):
    """Scrape bachelor's courses for a given program."""
    soup = fetch_and_parse_url(prog.url)
    courses_section = soup.find("div", id="suggestedsequencestextcontainer").find("table")
    if not courses_section:
        raise ValueError(f"Could not find courses section for {prog.name}.")

    second_natural_world = False

    semesters = {}
    current_semester, current_year, courses = None, None, []
    semester_idx = 1

    for tr in courses_section.find_all("tr"):
        if "plangridterm" in tr.get("class", []):  # Semester row
            if current_semester and current_year and courses:
                semester_idx = process_semester_courses(
                    semesters, semester_idx, current_year, current_semester, courses, prog.name
                )
                courses = []
            current_semester = tr.find("th").get_text(strip=True)
        elif "plangridyear" in tr.get("class", []):  # Year row
            if current_semester and current_year and courses:
                semester_idx = process_semester_courses(
                    semesters, semester_idx, current_year, current_semester, courses, prog.name
                )
                courses = []
            current_year = tr.find("th", class_="year").get_text(strip=True)
        elif "plangridsum" not in tr.get("class", []) and "plangridtotal" not in tr.get("class", []):  # Course row
            tds = tr.find_all("td")
            if len(tds) == 3:
                code = tds[0].get_text(strip=True)
                title = tds[1].get_text(strip=True)
                hours = tds[2].get_text(strip=True)
                if hours == "NULL":
                    hours = "0"
                elif not hours:
                    logger.warning("Skipping malformed course row: %s in %s", title, prog.name)
                    continue
                hours = int(hours)
                url = tds[0].find("a")["href"]
                courses.append(dataclasses.asdict(Course(CourseKind.DEGREE, title, hours, code, url)))
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
                    logger.warning("Skipping malformed course row: %s in %s", title, prog.name)
                    continue
                hours = int(hours)
                try:
                    gened = GenEds(title)
                except ValueError:
                    courses.append(dataclasses.asdict(Course(CourseKind.ELECTIVE_STUB, title, hours)))
                else:
                    if not IGNORE_GENED_STUBS:
                        courses.append(dataclasses.asdict(Course(CourseKind.GENED_STUB, title, hours, info=gened)))

    if current_semester and current_year and courses:
        semester_idx = process_semester_courses(
            semesters, semester_idx, current_year, current_semester, courses, prog.name
        )

    return pd.concat(semesters, axis=1)


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


def multiindex_df_to_column_grouped_xml(df):
    root = etree.Element("curriculum")

    # Loop through each semester
    for sem in df.columns.levels[0]:
        sem_elem = etree.SubElement(root, "semester", name=sem)

        # Loop through each row for this semester
        for i in df.index:
            try:
                values = df[sem].loc[i]
            except KeyError:
                continue  # Skip if semester missing from this row

            if pd.isna(values).all():
                continue  # Skip entirely empty rows

            course_elem = etree.SubElement(sem_elem, "course")
            for key, val in values.items():
                if pd.notna(val):  # Skip NaN values
                    if isinstance(val, float):
                        val = int(val)
                    sub_elem = etree.SubElement(course_elem, key)
                    sub_elem.text = str(val)

    return etree.tostring(root, pretty_print=True).decode()


def gened_df_to_xml(df):
    root = etree.Element("requirements")

    # Loop through each semester
    for sem in df.columns.levels[0]:
        sem_elem = etree.SubElement(root, "gened", name=sem)

        # Loop through each row for this semester
        for i in df.index:
            try:
                values = df[sem].loc[i]
            except KeyError:
                continue  # Skip if semester missing from this row

            if pd.isna(values).all():
                continue  # Skip entirely empty rows

            course_elem = etree.SubElement(sem_elem, "course")
            for key, val in values.items():
                if pd.notna(val):  # Skip NaN values
                    if isinstance(val, float):
                        val = int(val)
                    sub_elem = etree.SubElement(course_elem, key)
                    sub_elem.text = str(val)

    return etree.tostring(root, pretty_print=True).decode()


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
                logger.warning("Skipping malformed course row: %s in %s", title, gened.name)
                continue
            hours = int(hours)
            url = tds[0].find("a")["href"]
            yield dataclasses.asdict(
                Course(
                    CourseKind.GENED,
                    title,
                    hours,
                    code,
                    url,
                    gened,
                )
            )


def scrape_gened_courselists():
    gened_courses = {}
    for gened in GenEds:
        courses = []
        if gened.value.Url:
            # Construct the URL for the GenEd
            url = requests.compat.urljoin(SITE_URL, gened.value.Url)
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
                    dataclasses.asdict(
                        Course(
                            CourseKind.GENED,
                            details["title"],
                            details["credits"],
                            details["code"],
                            details["url"],
                            gened,
                        )
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
        gened_courses[gened.name] = pd.DataFrame(courses)

    return pd.concat(gened_courses, axis=1)


def course_lookup(code: str) -> dict[str]:
    url = requests.compat.urljoin(SITE_URL, "/search") + "?P=" + code.upper()
    soup = fetch_and_parse_url(url)
    course_section = soup.find("div", class_="courseblock")
    course_code = course_section.find("span", class_="detail-code").get_text(strip=True)
    title = course_section.find("span", class_="detail-title").get_text(strip=True)
    credits_raw = course_section.find("span", class_="detail-hours_html").get_text(strip=True)
    credits = credits_raw.strip("()").split()[0]  # Extract just the number

    return {"code": course_code, "title": title, "credits": int(credits), "url": url}


def inject(mode="debug"):
    with zipfile.ZipFile("scraped_programs/temp.zip", "w", compression=zipfile.ZIP_DEFLATED, compresslevel=9) as zf:
        for i in glob.glob("scraped_programs/*.xml"):
            zf.write(i, i)
    with open(f"target/{mode}/schedulebot.exe", "ab") as exe, open("scraped_programs/temp.zip", "rb") as zipobj:
        # exe.seek(0, os.SEEK_END)
        exe.write(zipobj.read())


def main():
    url = requests.compat.urljoin(SITE_URL, "/courses-instruction") + "#programstext"
    if IGNORE_GENED_STUBS:
        logger.warning("Gened stubs are being ignored, resulting programs will be missing required gened classes")
    try:
        geneds = scrape_gened_courselists()
        with open(
            pathlib.Path("scraped_programs").joinpath("General_Education.xml"),
            "w",
            encoding="utf-8",
        ) as file:
            file.write(gened_df_to_xml(geneds))
        geneds.to_excel("scraped_geneds.xlsx")
        programs = scrape_program_info(url)
        logger.info("Programs and their links:")
        with pd.ExcelWriter("scraped_programs.xlsx") as writer:
            for prog in programs:
                if prog.kind == ProgramKind.Bachelor:
                    try:
                        df = scrape_bachelors_courses(prog)
                        df.to_excel(writer, sheet_name=trim_titles(prog.name))
                        with open(
                            pathlib.Path("scraped_programs").joinpath(
                                pathvalidate.sanitize_filename(prog.name).replace(" ", "_") + ".xml"
                            ),
                            "w",
                            encoding="utf-8",
                        ) as f:
                            f.write(multiindex_df_to_column_grouped_xml(df))
                    except Exception as e:
                        logger.error("An error occurred: %s in %s", e, prog.name)
    except Exception as e:
        logger.error("An error occurred: %s", e)
        raise
    # inject()


if __name__ == "__main__":
    # main()
    inject()
