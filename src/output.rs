use colored::Colorize;

use crate::checks::CheckResult;

/// Print all check results and return (passed, failed) counts
pub fn print_summary(results: &[CheckResult]) -> (usize, usize) {
    let mut passed = 0;
    let mut failed = 0;

    for result in results {
        if result.passed {
            passed += 1;
            println!(
                "{} {}: {} - {}",
                "🌱",
                result.check_type.dimmed(),
                result.name.green(),
                result.message.dimmed()
            );
        } else {
            failed += 1;
            println!(
                "{} {}: {} - {}",
                "🥀",
                result.check_type.dimmed(),
                result.name.red(),
                result.message.yellow()
            );
        }
    }

    (passed, failed)
}
