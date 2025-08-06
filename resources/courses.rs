use crate::{schedule::CourseCode, CC};
use std::collections::HashMap;

pub fn courses() -> HashMap<CourseCode, (String, Option<u32>)> {
    let mut courses = HashMap::new();
    courses.insert(
        CC!("CHEM", 1200),
        ("General Chemistry I Lecture".into(), Some(3)),
    );
    courses.insert(
        CC!("CHEM", 1201),
        ("General Chemistry I Lab".into(), Some(1)),
    );
    courses.insert(CC!("MATH", 1300), ("Calculus I".into(), Some(4)));
    courses.insert(
        CC!("CHEM", 1210),
        ("General Chemistry II Lecture".into(), Some(3)),
    );
    courses.insert(
        CC!("CHEM", 1211),
        ("General Chemistry II Lab".into(), Some(1)),
    );
    courses.insert(CC!("MATH", 1350), ("Calculus II".into(), Some(4)));
    courses.insert(
        CC!("CHEM", 2200),
        ("Organic Chemistry I Lecture".into(), Some(3)),
    );
    courses.insert(
        CC!("CHEM", 2201),
        ("Organic Chemistry I Lab".into(), Some(1)),
    );
    courses.insert(CC!("PHYS", 2100), ("Classical Physics I".into(), Some(3)));
    courses.insert(
        CC!("PHYS", 2101),
        ("Introductory Physics Laborartory I".into(), Some(1)),
    );
    courses.insert(
        CC!("CHEM", 2210),
        ("Organic Chemistry II Lecture".into(), Some(3)),
    );
    courses.insert(CC!("CHEM", 2211), ("Organic Chem II Lab".into(), Some(1)));
    courses.insert(CC!("PHYS", 2110), ("Classical Physics II".into(), Some(3)));
    courses.insert(
        CC!("PHYS", 2111),
        ("Introductory Physics Lab II".into(), Some(1)),
    );
    courses.insert(CC!("CHEM", 3300), ("Quantitative Analysis".into(), Some(3)));
    courses.insert(
        CC!("CHEM", 3301),
        ("Quantitative Analysis Lab".into(), Some(1)),
    );
    courses.insert(CC!("CHEM", 3500), ("Biochemistry I".into(), Some(3)));
    courses.insert(CC!("CHEM", 3501), ("Biochemistry I Lab".into(), Some(1)));
    courses.insert(
        CC!("CHEM", 4900),
        ("Chemistry & Biochem Colloquium".into(), Some(1)),
    );
    courses.insert(
        CC!("CHEM", 3311),
        ("Instrumental Analysis Lab".into(), Some(1)),
    );
    courses.insert(CC!("CHEM", 3400), ("Inorganic Chemistry".into(), Some(3)));
    courses.insert(
        CC!("CHEM", 3401),
        ("Inorganic Chemistry Lab".into(), Some(1)),
    );
    courses.insert(CC!("CHEM", 3800), ("Physical Chemistry I".into(), Some(3)));
    courses.insert(
        CC!("CHEM", 3801),
        ("Physical Chemistry I Lab".into(), Some(1)),
    );
    courses.insert(
        CC!("CHEM", 4901),
        ("Chem & Biochem Colloquium 2".into(), Some(1)),
    );
    courses.insert(CC!("CHEM", 4801), ("Research I".into(), Some(1)));
    courses.insert(
        CC!("CHEM", 4902),
        ("Chem & Biochem Colloquium 3".into(), Some(1)),
    );
    courses.insert(CC!("CHEM", 4811), ("Research II".into(), Some(1)));
    courses.insert(
        CC!("CHEM", 4903),
        ("Chem & Biochem Colloquium 4".into(), Some(1)),
    );
    courses.insert(
        CC!("CHEM", "COMP"),
        ("Senior Comprehensive Exam".into(), None),
    );
    courses.insert(CC!("PHYS", 2100), ("Classical Physics I".into(), Some(3)));
    courses.insert(
        CC!("PHYS", 2101),
        ("Introductory Physics Laborartory I".into(), Some(1)),
    );
    courses.insert(
        CC!("CHEM", 1200),
        ("General Chemistry I Lecture".into(), Some(3)),
    );
    courses.insert(
        CC!("CHEM", 1201),
        ("General Chemistry I Lab".into(), Some(1)),
    );
    courses.insert(CC!("MATH", 1300), ("Calculus I".into(), Some(4)));
    courses.insert(CC!("PHYS", 2110), ("Classical Physics II".into(), Some(3)));
    courses.insert(
        CC!("PHYS", 2111),
        ("Introductory Physics Lab II".into(), Some(1)),
    );
    courses.insert(CC!("MATH", 1350), ("Calculus II".into(), Some(4)));
    courses.insert(
        CC!("PHYS", 3200),
        ("Relativity & Atomic Physics".into(), Some(3)),
    );
    courses.insert(CC!("PHYS", 3201), ("Modern Physics Lab".into(), Some(1)));
    courses.insert(CC!("MATH", 2300), ("Calculus III".into(), Some(4)));
    courses.insert(
        CC!("PHYS", 4200),
        ("Mathematical Methods for Physics".into(), Some(3)),
    );
    courses.insert(
        CC!("PHYS", 3210),
        ("Nuclear & Elementary Particle Physics".into(), Some(2)),
    );
    courses.insert(CC!("PHYS", 3211), ("Modern Physics Lab II".into(), Some(1)));
    courses.insert(
        CC!("MATH", 3100),
        ("Differential Equations".into(), Some(3)),
    );
    courses.insert(
        CC!("CSCI", 2300),
        ("Programming for Scientists & Engineers".into(), Some(3)),
    );
    courses.insert(CC!("PHYS", 4100), ("Mechanics I".into(), Some(3)));
    courses.insert(CC!("PHYS", 4900), ("Physics Colloquium".into(), None));
    courses.insert(CC!("PHYS", 4300), ("Optics".into(), Some(3)));
    courses.insert(CC!("PHYS", 4301), ("Optics Laboratory".into(), Some(1)));
    courses.insert(CC!("PHYS", 4901), ("Physics Colloquium".into(), None));
    courses.insert(
        CC!("PHYS", 4600),
        ("Electricity & Magnetism I".into(), Some(3)),
    );
    courses.insert(CC!("PHYS", 4902), ("Physics Colloquium".into(), None));
    courses.insert(CC!("PHYS", 4800), ("Quantum Mechanics".into(), Some(3)));
    courses.insert(
        CC!("PHYS", 4910),
        ("Physics & Astronomy Research".into(), Some(1)),
    );
    courses.insert(CC!("PHYS", 4903), ("Physics Colloquium".into(), None));
    courses.insert(
        CC!("PHYS", "COMP"),
        ("Senior Comprehensive Exam".into(), None),
    );
    courses.insert(
        CC!("CHEM", 3311),
        ("Instrumental Analysis Lab".into(), Some(1)),
    );
    courses.insert(CC!("CHEM", 3300), ("Quantitative Analysis".into(), Some(3)));
    courses.insert(
        CC!("CHEM", 3301),
        ("Quantitative Analysis Lab".into(), Some(1)),
    );
    courses.insert(CC!("CHEM", 3511), ("Biochemistry II Lab".into(), Some(1)));
    courses.insert(CC!("CHEM", 3510), ("Biochemistry II".into(), Some(3)));
    courses.insert(CC!("BIOL", 3345), ("Developmental Biology".into(), Some(4)));
    courses.insert(
        CC!("CHEM", 1210),
        ("General Chemistry II Lecture".into(), Some(3)),
    );
    courses.insert(CC!("BIOL", 4476), ("Immunology".into(), Some(4)));
    courses.insert(
        CC!("CHEM", 2200),
        ("Organic Chemistry I Lecture".into(), Some(3)),
    );
    courses.insert(CC!("CHEM", 3500), ("Biochemistry I".into(), Some(3)));
    courses.insert(
        CC!("CHEM", 2210),
        ("Organic Chemistry II Lecture".into(), Some(3)),
    );
    courses.insert(CC!("CHEM", 2211), ("Organic Chem II Lab".into(), Some(1)));
    courses.insert(CC!("CHEM", 3501), ("Biochemistry I Lab".into(), Some(1)));
    courses.insert(CC!("BIOL", 4486), ("Research".into(), Some(1)));
    courses.insert(
        CC!("CHEM", 1210),
        ("General Chemistry II Lecture".into(), Some(3)),
    );
    courses.insert(CC!("BIOL", 3355), ("Ecology".into(), Some(4)));
    courses.insert(
        CC!("CHEM", 1210),
        ("General Chemistry II Lecture".into(), Some(3)),
    );
    courses.insert(CC!("CHEM", 2211), ("Organic Chem II Lab".into(), Some(1)));
    courses.insert(
        CC!("CHEM", 2201),
        ("Organic Chemistry I Lab".into(), Some(1)),
    );
    courses.insert(
        CC!("CHEM", 2210),
        ("Organic Chemistry II Lecture".into(), Some(3)),
    );
    courses.insert(CC!("BIOL", 3360), ("Microbiology".into(), Some(4)));
    courses.insert(
        CC!("CHEM", 2200),
        ("Organic Chemistry I Lecture".into(), Some(3)),
    );
    courses.insert(CC!("BIOL", 3370), ("Genetics".into(), Some(4)));
    courses.insert(CC!("ASTR", 4300), ("Galaxies & Cosmology".into(), Some(3)));
    courses.insert(
        CC!("PHYS", 3200),
        ("Relativity & Atomic Physics".into(), Some(3)),
    );
    courses.insert(
        CC!("BIOL", 4475),
        ("Molecular & Cell Biology".into(), Some(4)),
    );
    courses.insert(
        CC!("CHEM", 2200),
        ("Organic Chemistry I Lecture".into(), Some(3)),
    );
    courses.insert(
        CC!("CHEM", 1201),
        ("General Chemistry I Lab".into(), Some(1)),
    );
    courses.insert(
        CC!("CHEM", 1200),
        ("General Chemistry I Lecture".into(), Some(3)),
    );
    courses.insert(CC!("BIOL", 4410), ("Cancer Biology".into(), Some(4)));
    courses.insert(
        CC!("CHEM", 1210),
        ("General Chemistry II Lecture".into(), Some(3)),
    );
    courses.insert(
        CC!("ASTR", 4200),
        ("Solar System Astrophysics".into(), Some(3)),
    );
    courses.insert(CC!("PHYS", 2110), ("Classical Physics II".into(), Some(3)));
    courses.insert(CC!("CHEM", 3300), ("Quantitative Analysis".into(), Some(3)));
    courses.insert(
        CC!("CHEM", 1210),
        ("General Chemistry II Lecture".into(), Some(3)),
    );
    courses.insert(
        CC!("CHEM", 1211),
        ("General Chemistry II Lab".into(), Some(1)),
    );
    courses.insert(
        CC!("CHEM", 3301),
        ("Quantitative Analysis Lab".into(), Some(1)),
    );
    courses.insert(CC!("BIOL", 3312), ("Plant Biology".into(), Some(4)));
    courses.insert(
        CC!("CHEM", 1210),
        ("General Chemistry II Lecture".into(), Some(3)),
    );
    courses.insert(
        CC!("CHEM", 3301),
        ("Quantitative Analysis Lab".into(), Some(1)),
    );
    courses.insert(CC!("CHEM", 3300), ("Quantitative Analysis".into(), Some(3)));
    courses.insert(CC!("CHEM", 3510), ("Biochemistry II".into(), Some(3)));
    courses.insert(CC!("CHEM", 3500), ("Biochemistry I".into(), Some(3)));
    courses.insert(CC!("CHEM", 3511), ("Biochemistry II Lab".into(), Some(1)));
    courses.insert(
        CC!("CHEM", 1010),
        ("Chemistry of the Biosphere".into(), Some(3)),
    );
    courses.insert(
        CC!("CHEM", 1011),
        ("Chemistry of the Biosphere Lab".into(), Some(1)),
    );
    courses.insert(
        CC!("CHEM", 1210),
        ("General Chemistry II Lecture".into(), Some(3)),
    );
    courses.insert(
        CC!("CHEM", 1201),
        ("General Chemistry I Lab".into(), Some(1)),
    );
    courses.insert(
        CC!("CHEM", 1211),
        ("General Chemistry II Lab".into(), Some(1)),
    );
    courses.insert(
        CC!("BIOL", 3310),
        ("Biology III- Mechanisms of Evolution".into(), Some(3)),
    );
    courses.insert(CC!("BIOL", 1121), ("General Biology I".into(), Some(5)));
    courses.insert(CC!("BIOL", 1122), ("General Biology II".into(), Some(4)));
    courses.insert(
        CC!("BIOL", 3347),
        ("Kansas Vertebrates Natural History".into(), Some(4)),
    );
    courses.insert(
        CC!("CHEM", 1210),
        ("General Chemistry II Lecture".into(), Some(3)),
    );
    courses.insert(
        CC!("BIOL", 3313),
        ("Taxonomy of Flowering Plants".into(), Some(4)),
    );
    courses.insert(
        CC!("CHEM", 1210),
        ("General Chemistry II Lecture".into(), Some(3)),
    );
    courses.insert(
        CC!("CHEM", 2200),
        ("Organic Chemistry I Lecture".into(), Some(3)),
    );
    courses.insert(
        CC!("CHEM", 1210),
        ("General Chemistry II Lecture".into(), Some(3)),
    );
    courses.insert(
        CC!("CHEM", 1211),
        ("General Chemistry II Lab".into(), Some(1)),
    );
    courses.insert(
        CC!("CHEM", 2201),
        ("Organic Chemistry I Lab".into(), Some(1)),
    );
    courses.insert(
        CC!("CHEM", 3250),
        ("Environmental Chemistry".into(), Some(3)),
    );
    courses.insert(
        CC!("CHEM", 2200),
        ("Organic Chemistry I Lecture".into(), Some(3)),
    );
    courses.insert(
        CC!("ASTR", 4100),
        ("Introduction to Astrophysics".into(), Some(3)),
    );
    courses.insert(
        CC!("PHYS", 3200),
        ("Relativity & Atomic Physics".into(), Some(3)),
    );
    courses.insert(CC!("BIOL", 3354), ("Animal Behavior".into(), Some(4)));
    courses.insert(
        CC!("CHEM", 1210),
        ("General Chemistry II Lecture".into(), Some(3)),
    );
    courses.insert(CC!("BIOL", 3305), ("Biological Statistics".into(), Some(4)));
    courses.insert(CC!("BIOL", 1121), ("General Biology I".into(), Some(5)));
    courses.insert(CC!("BIOL", 1122), ("General Biology II".into(), Some(4)));
    courses.insert(
        CC!("CHEM", 1011),
        ("Chemistry of the Biosphere Lab".into(), Some(1)),
    );
    courses.insert(
        CC!("CHEM", 1010),
        ("Chemistry of the Biosphere".into(), Some(3)),
    );
    courses.insert(
        CC!("CHEM", 1200),
        ("General Chemistry I Lecture".into(), Some(3)),
    );
    courses.insert(
        CC!("CHEM", 1201),
        ("General Chemistry I Lab".into(), Some(1)),
    );
    courses.insert(CC!("BIOL", 4484), ("Cell Physiology".into(), Some(4)));
    courses.insert(
        CC!("CHEM", 2200),
        ("Organic Chemistry I Lecture".into(), Some(3)),
    );
    courses.insert(CC!("CHEM", 3501), ("Biochemistry I Lab".into(), Some(1)));
    courses.insert(CC!("CHEM", 3500), ("Biochemistry I".into(), Some(3)));
    courses.insert(
        CC!("ASTR", 3000),
        ("Observational Astronomy".into(), Some(3)),
    );
    courses.insert(CC!("PHYS", 2110), ("Classical Physics II".into(), Some(3)));
    courses.insert(
        CC!("BIOL", 2260),
        ("Principles of Microbiology".into(), Some(4)),
    );
    courses.insert(CC!("BIOL", 1121), ("General Biology I".into(), Some(5)));
    courses.insert(CC!("BIOL", 3370), ("Genetics".into(), Some(4)));
    courses.insert(
        CC!("CHEM", 1210),
        ("General Chemistry II Lecture".into(), Some(3)),
    );
    courses.insert(
        CC!("CHEM", 1211),
        ("General Chemistry II Lab".into(), Some(1)),
    );
    courses.insert(
        CC!("CHEM", 1210),
        ("General Chemistry II Lecture".into(), Some(3)),
    );
    courses.insert(CC!("CHEM", 3400), ("Inorganic Chemistry".into(), Some(3)));
    courses.insert(
        CC!("CHEM", 2210),
        ("Organic Chemistry II Lecture".into(), Some(3)),
    );
    courses.insert(CC!("CHEM", 2211), ("Organic Chem II Lab".into(), Some(1)));
    courses.insert(
        CC!("CHEM", 3401),
        ("Inorganic Chemistry Lab".into(), Some(1)),
    );
    courses.insert(
        CC!("CHEM", 3150),
        ("Computational Chemistry".into(), Some(3)),
    );
    courses.insert(
        CC!("CHEM", 1210),
        ("General Chemistry II Lecture".into(), Some(3)),
    );
    courses.insert(CC!("MATH", 1350), ("Calculus II".into(), Some(4)));
    courses.insert(CC!("PHYS", 2110), ("Classical Physics II".into(), Some(3)));
    courses.insert(CC!("BIOL", 3353), ("Invertebrate Biology".into(), Some(4)));
    courses.insert(
        CC!("CHEM", 1210),
        ("General Chemistry II Lecture".into(), Some(3)),
    );
    courses.insert(
        CC!("BIOL", 3346),
        ("Comparative Vertebrate Anatomy".into(), Some(4)),
    );
    courses.insert(
        CC!("CHEM", 1210),
        ("General Chemistry II Lecture".into(), Some(3)),
    );
    courses.insert(CC!("BIOL", 4482), ("Animal Physiology".into(), Some(4)));
    courses.insert(
        CC!("CHEM", 1210),
        ("General Chemistry II Lecture".into(), Some(3)),
    );
    courses.insert(
        CC!("CHEM", 2201),
        ("Organic Chemistry I Lab".into(), Some(1)),
    );
    courses.insert(
        CC!("CHEM", 2200),
        ("Organic Chemistry I Lecture".into(), Some(3)),
    );
    courses.insert(
        CC!("CHEM", 2210),
        ("Organic Chemistry II Lecture".into(), Some(3)),
    );
    courses.insert(
        CC!("CHEM", 2200),
        ("Organic Chemistry I Lecture".into(), Some(3)),
    );
    courses.insert(CC!("CHEM", 2211), ("Organic Chem II Lab".into(), Some(1)));
    courses
}
