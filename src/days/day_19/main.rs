use itertools::Itertools;
use crate::days::day_19::report::Report;
use crate::{parse_input, Part};
use crate::days::day_19::vector::Vector;
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day19_launch(part: Part) -> AOCResult<String> {
    let full_report = create_full_report(false)?;
    match part {
        Part::Part1 => part1(&full_report),
        Part::Part2 => part2(&full_report, false)
    }
}

fn part1(full_report: &Report) -> AOCResult<String> {
    Ok(full_report.nb_beacons().to_string())
}

fn create_full_report(for_test: bool) -> AOCResult<Report> {
    let mut reports = parse_input(for_test)?;

    while let Some((i, j, report)) = find_matching_reports(&reports) {
        reports.remove(j);
        reports[i].merge(report)
    }
    Ok(reports.remove(0))
}


fn part2(full_report: &Report, for_test: bool) -> AOCResult<String> {
    let reports = parse_input(for_test)?;

    let offsets: Vec<Vector> = reports.iter()
        .map(|r| full_report.find_match(r))
        .flatten()
        .map(|m| m.offset())
        .collect();

    let nb_offsets = offsets.len();

    (0..nb_offsets).cartesian_product(0..nb_offsets)
        .filter(|(i,j)| i<j)
        .map(|(i,j)| offsets[i].manhattan_distance(&offsets[j]))
        .max()
        .ok_or_else(|| "no offset found".to_string())
        .map(|max_distance| max_distance.to_string())

}


fn find_matching_reports(reports: &[Report]) -> Option<(usize, usize, Report)> {
    let nb_reports = reports.len();

    (0..nb_reports).cartesian_product(0..nb_reports)
        .filter(|(i, j)| j > i)
        .find_map(|(i, j)| find_match(reports, i, j))
}


fn find_match(reports: &[Report], report_idx1: usize, report_idx2: usize) -> Option<(usize, usize, Report)> {
    reports[report_idx1].find_match(&reports[report_idx2])
        .map(|m| (report_idx1, report_idx2, m.report()))
}


#[allow(dead_code)]
fn parse_input(for_test: bool) -> AOCResult<Vec<Report>> {
    Ok(Problem::factory(for_test)(19)
        .read_input()?
        .split("\n\n")
        .map(|r| parse_input!(r, Report))
        .collect())
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_19::main::{create_full_report, parse_input, part1, part2};
    use crate::days::day_19::report::Report;


    #[test]
    fn day19_part1_test() {
        let full_report = create_full_report(true).unwrap();
        let result = part1(&full_report).unwrap();
        assert_eq!(result, "79")
    }

    #[test]
    fn day19_part2_test() {
        let full_report = create_full_report(true).unwrap();
        let result = part2(&full_report, true).unwrap();
        assert_eq!(result, "3621")
    }
}