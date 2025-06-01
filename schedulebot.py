import requests
from bs4 import BeautifulSoup
import requests_cache

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
    programs_section = soup.find("div", id="programsbycredentialtextcontainer")
    if not programs_section:
        raise ValueError("Could not find the programs section on the page.")

    # Extract all links within the programs section
    program_links = programs_section.find_all("a", href=True)
    programs = {
        link.text.strip(): requests.compat.urljoin(url, link["href"])
        for link in program_links
    }

    return programs


def main():
    url = "https://coursecatalog.benedictine.edu/courses-instruction/#programstext"
    try:
        programs = scrape_program_links(url)
        print("Programs and their links:")
        for name, link in programs.items():
            print(f"{name}: {link}")
    except Exception as e:
        print(f"An error occurred: {e}")


if __name__ == "__main__":
    main()
