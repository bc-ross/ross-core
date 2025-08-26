import json
import logging
import random
import subprocess
import time

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


def course_lookup(code: str) -> dict[str]:
    url = requests.compat.urljoin(SITE_URL, "/search") + "?P=" + code.upper()
    soup = fetch_and_parse_url(url)
    course_section = soup.find("div", class_="courseblock")
    course_code = course_section.find("span", class_="detail-code").get_text(strip=True)
    title = course_section.find("span", class_="detail-title").get_text(strip=True)
    credits_raw = course_section.find("span", class_="detail-hours_html").get_text(strip=True)
    credits = credits_raw.strip("()").split()[0]  # Extract just the number
    term_raw = course_section.find("span", class_="detail-term").get_text(strip=True)
    term = term_raw.strip("()").split()[0]

    return {
        "code": course_code,
        "title": title,
        "credits": int(credits) if credits.upper().strip() != "NULL" else None,
        "url": url,
        "term": term,
    }


def format_code(course) -> str:
    return course["stem"].upper() + "-" + str(list(course["code"].values())[0]).upper()


PREAMBLE = """
use crate::{CC, schedule::{CourseCode, CourseTermOffering::{self, *}}};
use std::collections::HashMap;

pub fn courses() -> HashMap<CourseCode, (String, Option<u32>, CourseTermOffering)> {
    HashMap::from([
"""

POSTAMBLE = "])\n}\n"


def repr_rs(x: str) -> str:
    return repr(x).replace("'", '"')


def format_course(course, assoc_info) -> str:
    title, credits, term = assoc_info
    title = title.replace('"', '\\"')
    if credits is None:
        credits_str = "None"
    else:
        credits_str = f"Some({credits})"

    return f"({format_cc(course)} ({repr_rs(title)}.into(), {credits_str}, {term.title()})),\n"


def format_cc(course) -> str:
    stem, code = course.split("-")
    if code.isnumeric():
        code = int(code)

    return f"CC!({repr_rs(stem)}, {repr_rs(code)}),"


def main():
    with open("../script_assistant/new_courses.json") as file:
        courses = json.load(file)
    with open("../script_assistant/old_courses.json") as file:
        export_courses = json.load(file)
    for course in courses:
        code = format_code(course)
        try:
            course_info = course_lookup(code)
            print(
                f"Course: {course_info['code']}, Title: {course_info['title']}, Credits: {course_info['credits']}, Term: {course_info['term']}, URL: {course_info['url']}"
            )
            export_courses.append(
                (format_code(course), (course_info["title"], course_info["credits"], course_info["term"]))
            )
        except requests.RequestException as e:
            logger.error("Failed to fetch course %s: %s", code, e)
        except Exception as e:
            logger.error("An error occurred while processing course %s: %s", code, e)
        else:
            time.sleep(random.randrange(1, 15) / 100)  # Sleep to avoid overwhelming the server
    with open("../resources/courses.rs", "w", encoding="utf-8") as f:
        f.write(PREAMBLE)
        for course, assoc_info in sorted(export_courses, key=lambda x: x[0]):
            f.write(format_course(course, assoc_info))
        f.write(POSTAMBLE)
    subprocess.run(["rustfmt", "../resources/courses.rs"], check=True)


if __name__ == "__main__":
    main()
