import requests
from bs4 import BeautifulSoup
import pandas as pd
import requests_cache
from data_structures import ProgramStub, ProgramKind, DegreeCourse

# Enable caching
requests_cache.install_cache(
    "schedulebot_cache", expire_after=86400
)  # Cache expires after 1 day


def scrape_program_info(url):
    response = requests.get(url, timeout=10)
    response.raise_for_status()  # Raise an error for bad status codes
    soup = BeautifulSoup(response.text, "html.parser")
    # Check if the response was served from cache
    if response.from_cache:
        print("Response was served from cache.")
    else:
        print("Response was fetched from the server.")
    # Find the list of programs
    programs_section = soup.find("div", id="programstextcontainer").find("tbody")
    if not programs_section:
        raise ValueError("Could not find the programs section on the page.")

    programs = []

    # Extract all links within the programs section
    for row in programs_section.find_all("tr"):
        data = {}
        for i, td in enumerate(row.find_all("td")):
            spans = td.find_all("span")
            if len(spans) >= 2:
                key = spans[0].get_text(strip=True)
                value = spans[1].get_text(strip=True)
                data[key] = value
                # If it's the Program column, extract the URL
                if i == 0:
                    a_tag = spans[1].find("a")
                    if a_tag and a_tag.has_attr("href"):
                        data["URL"] = a_tag["href"]
        if data and data["Level"] == "UG":  # Filter out Master's
            specialization = False
            if data["Degree Type"] == "Bachelor with Specialization":
                specialization = True
                data["Degree Type"] = "Bachelor"
            programs.append(
                ProgramStub(
                    data["Program"],
                    ProgramKind(data["Degree Type"]),
                    requests.compat.urljoin(url, data["URL"])
                    + "#suggestedsequencestext",
                    specialization,
                )
            )
    return programs


def scrape_bachelors_courses(prog: ProgramStub):
    response = requests.get(prog.url, timeout=10)
    response.raise_for_status()  # Raise an error for bad status codes
    soup = BeautifulSoup(response.text, "html.parser")

    # Check if the response was served from cache
    if response.from_cache:
        print(f"Courses for {prog.name} were served from cache.")
    else:
        print(f"Courses for {prog.name} were fetched from the server.")

    courses = []

    # Find the courses section
    courses_section = soup.find("div", id="suggestedsequencestextcontainer").find(
        "table"
    )
    if not courses_section:
        raise ValueError(f"Could not find courses section for {prog.name}.")

    semesters = {}
    current_semester = None
    current_year = None
    courses = []

    for tr in courses_section.find_all("tr"):
        # Detect semester row by class
        if "plangridterm" in tr.get("class", []):
            # Save previous semester courses if any
            if current_semester and current_year and courses:
                if current_year in [
                    "Freshman Year",
                    "Sophomore Year",
                    "Junior Year",
                    "Senior Year",
                ]:
                    # df = pd.DataFrame(courses, columns=["Code", "Title", "Credits"])
                    semesters[current_year + ": " + current_semester] = pd.Series(
                        courses
                    )
                    courses = []
                else:
                    print("Skipping non-standard year:", current_year)

            # Extract semester name (usually in <th>)
            current_semester = tr.find("th").get_text(strip=True)
        elif "plangridyear" in tr.get("class", []):
            # Save previous semester courses if any
            if current_semester and current_year and courses:
                if current_year in [
                    "Freshman Year",
                    "Sophomore Year",
                    "Junior Year",
                    "Senior Year",
                ]:
                    # df = pd.DataFrame(courses, columns=["Code", "Title", "Credits"])
                    semesters[current_year + ": " + current_semester] = pd.Series(
                        courses
                    )
                    courses = []
                else:
                    print(
                        "Skipping non-standard year:",
                        current_year,
                        "for program",
                        prog.name,
                    )

            # Extract year name (usually in <th>)
            current_year = tr.find("th", class_="year").get_text(strip=True)
        elif "plangridsum" not in tr.get("class", []) and "plangridtotal" not in tr.get(
            "class", []
        ):
            # Normal course rows have 3 <td>
            tds = tr.find_all("td")
            if len(tds) == 3:
                code = tds[0].get_text(strip=True)
                title = tds[1].get_text(strip=True)
                hours = tds[2].get_text(strip=True)
                url = tds[0].find("a")["href"]
                courses.append(DegreeCourse(title, code, hours, url))

    # Add the last semester courses after loop
    if current_semester and current_year and courses:
        if current_year in [
            "Freshman Year",
            "Sophomore Year",
            "Junior Year",
            "Senior Year",
        ]:
            # df = pd.DataFrame(courses, columns=["Code", "Title", "Credits"])
            semesters[current_year + ": " + current_semester] = pd.Series(courses)
        else:
            print("Skipping non-standard year:", current_year)

    sem_df = pd.DataFrame(semesters)
    return sem_df


def main():
    url = "https://coursecatalog.benedictine.edu/courses-instruction/#programstext"
    try:
        programs = scrape_program_info(url)
        print("Programs and their links:")
        with pd.ExcelWriter("programs.xlsx") as writer:
            for prog in programs:
                if prog.kind == ProgramKind.Bachelor:
                    scrape_bachelors_courses(prog).to_excel(writer, prog.name)
                    # break
    except Exception as e:
        print(f"An error occurred: {e}")
        raise


if __name__ == "__main__":
    main()
