use std::fs;

fn main() {
    let input = read_input();
    println!("Part 1: {}", containment_counts(&input));
    println!("Part 2: {}", overlap_counts(&input));
}

struct SectionAssignment {
    start: u32,
    end: u32
}

fn containment_counts(input: &str) -> u32 {
    input.lines()
        .map(section_assignments)
        .map(|section_assignments| {
            has_containment(section_assignments.0, section_assignments.1)
        })
        .filter(|&containment| containment)
        .count() as u32
}

fn overlap_counts(input: &str) -> u32 {
    input.lines()
        .map(section_assignments)
        .map(|section_assignments| {
            has_overlap(section_assignments.0, section_assignments.1)
        })
        .filter(|&overlap| overlap)
        .count() as u32
}

fn has_containment(first_section: SectionAssignment, second_section: SectionAssignment) -> bool {
    contains(&first_section, &second_section)
        || contains(&second_section, &first_section)
}

fn contains(outer: &SectionAssignment, inner: &SectionAssignment) -> bool {
    outer.start <= inner.start && outer.end >= inner.end
}

fn has_overlap(first_section: SectionAssignment, second_section: SectionAssignment) -> bool {
    overlaps(&first_section, &second_section)
        || overlaps(&second_section, &first_section)
}

fn overlaps(left: &SectionAssignment, right: &SectionAssignment) -> bool {
    left.start <= right.start && left.end >= right.start
}

fn section_assignments(elf_pair: &str) -> (SectionAssignment, SectionAssignment) {
    let elf_pair = elf_pair.split_once(',')
        .expect("no elf separator ',' found");
    let first_elf = elf_sections(elf_pair.0);
    let second_elf = elf_sections(elf_pair.1);
    (first_elf, second_elf)
}

fn elf_sections(elf: &str) -> SectionAssignment {
    let elf_sections = elf.split_once('-')
        .expect("no section separator '-' found");
    SectionAssignment {
        start: section_part(elf_sections.0),
        end: section_part(elf_sections.1)
    }
}

fn section_part(part: &str) -> u32 {
    part.parse().expect(format!("section part not a number: {}", part).as_str())
}

fn read_input() -> String {
    fs::read_to_string("input.txt")
        .expect("Could not read file")
}