// Este teste avalia:
// - Genéricos
// - Traits
//
// Uma escola mágica imaginária tem um novo sistema de geração de boletins escrito
// em Rust! Atualmente, o sistema só suporta a criação de boletins onde a
// nota do aluno é representada numericamente (ex.: 1.0 -> 5.5). No entanto, a
// escola também emite notas alfabéticas (A+ -> F-) e precisa ser capaz de
// imprimir ambos os tipos de boletim!

//
// Faça as alterações de código necessárias na struct `ReportCard` e no bloco de implementação
// para suportar boletins alfabéticos, além dos numéricos.

// TODO: Ajuste a struct conforme descrito acima.
struct ReportCard {
    grade: String,
    student_name: String,
    student_age: u8,
}

// TODO: Ajuste o bloco de implementação conforme descrito acima.
impl ReportCard {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade,
        )
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: "2.1".to_string(),
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+".to_string(),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}
