import requests
from bs4 import BeautifulSoup
import pandas as pd
import requests_cache
from data_structures import ProgramStub, ProgramKind, DegreeCourse

# Enable caching
requests_cache.install_cache(
    "schedulebot_cache", expire_after=86400
)  # Cache expires after 1 day


def fetch_and_parse_url(url):
    """Fetch a URL and parse it with BeautifulSoup."""
    response = requests.get(url, timeout=10)
    response.raise_for_status()
    soup = BeautifulSoup(response.text, "html.parser")
    if response.from_cache:
        print(f"Response for {url} was served from cache.")
    else:
        print(f"Response for {url} was fetched from the server.")
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
        print(f"Skipping non-standard year: {current_year} for program {prog_name}")


def scrape_bachelors_courses(prog: ProgramStub):
    """Scrape bachelor's courses for a given program."""
    soup = fetch_and_parse_url(prog.url)
    courses_section = soup.find("div", id="suggestedsequencestextcontainer").find(
        "table"
    )
    if not courses_section:
        raise ValueError(f"Could not find courses section for {prog.name}.")

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
                hours = int(tds[2].get_text(strip=True))
                url = tds[0].find("a")["href"]
                courses.append(DegreeCourse(title, code, hours, url))

    if current_semester and current_year and courses:
        process_semester_courses(
            semesters, current_year, current_semester, courses, prog.name
        )

    return pd.DataFrame(semesters)


def main():
    url = "https://coursecatalog.benedictine.edu/courses-instruction/#programstext"
    try:
        programs = scrape_program_info(url)
        print("Programs and their links:")
        with pd.ExcelWriter("programs.xlsx") as writer:
            for prog in programs:
                if prog.kind == ProgramKind.Bachelor:
                    scrape_bachelors_courses(prog).to_excel(writer, prog.name)
    except Exception as e:
        print(f"An error occurred: {e}")
        raise


if __name__ == "__main__":
    main()
