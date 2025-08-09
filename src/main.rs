use std::{collections::HashMap, fs::File, path::Path};
use std::io::prelude::*;
mod pretty;
use serde::{Deserialize, Serialize};
use ureq;
use chrono;

#[derive(Serialize, Deserialize)]
struct Sets {
    round_id: i32,
    station_id: String,
    name: String,
    dep: String,
    job: String,
    is_com: bool,
    server: String
}

#[derive(Serialize, Deserialize)]
struct Requ {
    map: Option<String>,
    name: String,
    panic_bunker: bool,
    players: u32,
    preset: String,
    round_id: i32,
    round_start_time: String,
    run_level: i32,
    soft_max_players: u32,
    tags: Vec<String>
}

fn main() {
    let depart: Vec<(String, String)> = vec![
        ("cc".to_string(), "ЦК".to_string()),
        ("com".to_string(), "Командование".to_string()),
        ("jur".to_string(), "Юр. департамент".to_string()),
        ("ss".to_string(), "Служба безопасности".to_string()),
        ("med".to_string(), "Медицинский отдел".to_string()),
        ("eng".to_string(), "Инженерный отдел".to_string()),
        ("rnd".to_string(), "Научный отдел".to_string()),
        ("car".to_string(), "Карго".to_string()),
        ("ser".to_string(), "Сервисный отдел".to_string()),
        ("ccc".to_string(), "Связь с ЦК".to_string())
    ];

    let mut jobs = HashMap::new();
    jobs.insert(
        "com",
        vec![
            ("cap".to_string(), "Капитан".to_string())
        ]
    );

    jobs.insert(
        "jur",
        vec![
            ("avd".to_string(), "Агент внутренних дел".to_string()),
            ("mag".to_string(), "Магистрат".to_string())
        ]
    );

    jobs.insert(
        "sec",
        vec![
            ("hos".to_string(), "Глава службы безопасности".to_string()),
            ("war".to_string(), "Смотритель".to_string()),
            ("ofi".to_string(), "Офицер службы безопасности".to_string()),
            ("det".to_string(), "Детектив".to_string()),
            ("pil".to_string(), "Пилот службы безопасности".to_string()),
            ("cad".to_string(), "Кадет службы безопасности".to_string())
        ]
    );

    jobs.insert(
        "med",
        vec![
            ("hom".to_string(), "Главный врач".to_string()),
            ("che".to_string(), "Химик".to_string()),
            ("doc".to_string(), "Врач".to_string()),
            ("par".to_string(), "Парамедик".to_string()),
            ("psy".to_string(), "Психолог".to_string()),
            ("int".to_string(), "Интерн".to_string())
        ]
    );

    jobs.insert(
        "rnd",
        vec![
            ("rd".to_string(), "Научный руководитель".to_string()),
            ("sci".to_string(), "Учёный".to_string()),
            ("stu".to_string(), "Научный ассистент".to_string())
        ]
    );

    jobs.insert(
        "eng",
        vec![
            ("ce".to_string(), "Старший инженер".to_string()),
            ("at".to_string(), "Атмосферный техник".to_string()),
            ("eng".to_string(), "Инженер".to_string()),
            ("tas".to_string(), "Технический ассистент".to_string())
        ]
    );

    jobs.insert(
        "car",
        vec![
            ("qua".to_string(), "Квартирмейстер".to_string()),
            ("sas".to_string(), "Утилизатор".to_string()),
            ("cat".to_string(), "Грузчик".to_string())
        ]
    );

    jobs.insert(
        "ser",
        vec![
            ("hop".to_string(), "Глава Персонала".to_string()),
            ("chf".to_string(), "Шеф-повар".to_string()),
            ("bot".to_string(), "Ботаник".to_string()),
            ("bar".to_string(), "Бармен".to_string()),
            ("rbt".to_string(), "Сервисный работник".to_string()),
            ("box".to_string(), "Боксёр".to_string()),
            ("cle".to_string(), "Уборщик".to_string()),
            ("bib".to_string(), "Библиотекарь".to_string()),
            ("hol".to_string(), "Священник".to_string()),
            ("zoo".to_string(), "Зоотехник".to_string()),
            ("rep".to_string(), "Репортёр".to_string()),
            ("mus".to_string(), "Музыкант".to_string()),
            ("grt".to_string(), "Пассажир".to_string()),
            ("mim".to_string(), "Мим".to_string()),
            ("clo".to_string(), "Клоун".to_string()),
        ]
    );
/*
    jobs.insert(
        "",
        vec![
            ("".to_string(), "".to_string()),
        ]
    );
*/
    let sets_path = Path::new("sets.json");
    let display = sets_path.display();

    let mut sets_file = File::open(&sets_path).unwrap();

    let mut sets_string = String::new();

    match sets_file.read_to_string(&mut sets_string) {
        Ok(_) => println!("Файл настроек успешно прочитан"),
        Err(why) => panic!("НЕ МОГУ НАСТРЫ ОТКРЫТЬ ААААА: {}", why)
    }

    let mut sets: Sets = serde_json::from_str(&sets_string).unwrap();
    let recv_body = ureq::get(format!("https://api.codetabs.com/v1/proxy/?quest=https://game2.station14.ru/{}/server/status", sets.server))
        .call()
        .unwrap()
        .body_mut()
        .read_to_string()
        .unwrap();
    let recv: Requ = serde_json::from_str(&recv_body).unwrap();

    if recv.round_id != sets.round_id {
        if pretty::y_or_n("Пробуксовка ебала! Стабилизировать?") {
            sets.round_id = recv.round_id;
            println!("Не буксуем");
            sets.name = pretty::asker("Чьих будешь?");
            sets.dep = pretty::choose("А где батрачишь?".to_string(), depart.clone());
            sets.is_com = pretty::y_or_n("Командование?");
            sets.station_id = pretty::asker("А какая станция?");
            let yeet = serde_json::to_string_pretty(&sets).unwrap();
            println!("{}", yeet);
        }
    }

    let now = chrono::Utc::now().naive_utc();
    let time_start = chrono::DateTime::parse_from_rfc3339(recv.round_start_time.as_str()).unwrap().naive_utc();
    let dury = (now - time_start).to_std().unwrap().as_secs();

    println!("{}", sets.round_id);
    println!("{}", recv_body);
    println!("{}", dury);
}
