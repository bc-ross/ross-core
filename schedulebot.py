import requests
from bs4 import BeautifulSoup
import requests_cache
from data_structures import ProgramStub, ProgramKind

# Enable caching
requests_cache.install_cache(
    "schedulebot_cache", expire_after=86400
)  # Cache expires after 1 day


def scrape_program_links(url):
    response = requests.get(url)
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
                    requests.compat.urljoin(url, data["URL"]),
                    specialization,
                )
            )
    return programs


def main():
    url = "https://coursecatalog.benedictine.edu/courses-instruction/#programstext"
    try:
        programs = scrape_program_links(url)
        print("Programs and their links:")
        for prog in programs:
            print(prog)
    except Exception as e:
        print(f"An error occurred: {e}")


if __name__ == "__main__":
    main()
