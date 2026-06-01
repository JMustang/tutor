// É fornecida uma lista de placares (um por linha) de uma partida de futebol.
// Cada linha tem o formato "<nome_do_time_1>,<nome_do_time_2>,<gols_do_time_1>,<gols_do_time_2>"
// Exemplo: "Inglaterra,França,4,2" (Inglaterra marcou 4 gols, França 2).
// Você deve construir uma tabela de placares contendo o nome do time,
//o total de gols marcados e o total de gols sofridos.

use std::collections::HashMap;

// Uma estrutura para armazenar os detalhes dos objetivos de uma equipe.
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    // O nome da equipe é a chave e sua estrutura associada é o valor.
    let mut scores = HashMap::<&str, TeamScores>::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // NOTE: Usamos `unwrap` porque ainda não lidamos com o tratamento de erros.
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // TODO: Preencha a tabela de pontuação com os detalhes extraídos.
        // TODO: Lembre-se de que os gols marcados pelo time 1 serão o número de gols
        // TODO: sofridos pelo time 2. Da mesma forma, os gols marcados pelo time 2 serão o
        // TODO: número de gols sofridos pelo time 1.

        // Time 1
        let team_1_entry = scores.entry(team_1_name).or_default();
        team_1_entry.goals_scored += team_1_score;
        team_1_entry.goals_conceded += team_2_score;

        // TIme 2
        let team_2_entry = scores.entry(team_2_name).or_default();
        team_2_entry.goals_scored += team_2_score;
        team_2_entry.goals_conceded += team_1_score;
    }

    scores
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(
            ["England", "France", "Germany", "Italy", "Poland", "Spain"]
                .into_iter()
                .all(|team_name| scores.contains_key(team_name))
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
