import logging
import sys

import requests
import requests_cache
from bs4 import BeautifulSoup

# Configure logging
logging.basicConfig(level=logging.WARNING, format="%(asctime)s - %(levelname)s - %(message)s")
logger = logging.getLogger(__name__)

# Enable caching
requests_cache.install_cache("schedulebot_cache", expire_after=86400)  # Cache expires after 1 day

SITE_URL = "https://coursecatalog.benedictine.edu"


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


def gened_lookup(gened: str) -> list[str]:
    url = requests.compat.urljoin(SITE_URL, "/general-education") + "/" + gened.lower().replace(" ", "-")
    print(url)
    soup = fetch_and_parse_url(url)
    course_section = soup.find("table", class_="sc_courselist")
    courses_raw = course_section.find_all("td", class_="codecol")
    courses = [i.get_text(strip=True).strip("()") for i in courses_raw if i.get_text(strip=True)]

    return courses


def repr_rs(x: str) -> str:
    return repr(x).replace("'", '"')


def format_cc(course) -> str:
    stem, code = course.split("-")
    if code.isnumeric():
        code = int(code)

    return f"CC!({repr_rs(stem)}, {repr_rs(code)}),"


def main():
    courses = gened_lookup(sys.argv[1])
    print(*[format_cc(course) for course in courses])


if __name__ == "__main__":
    main()
