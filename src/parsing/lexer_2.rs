pub fn baca(
    terima: std::sync::mpsc::Receiver<(
        /*nomer baris*/ u64,
        /*nama file*/ String,
        /*nilai*/ String,
    )>,
    kirim: std::sync::mpsc::Sender<(u64, String, String)>,
) {
    // tokenizer 2.1.0
    //let mut _str = (false,false,false);
    //let mut blok = (false,0u64,0u64,0u64);
    //fn str_() {}
    'main: loop {
        let (no_baris, nama_file, mut data) = terima.recv().unwrap();
        if !data.is_empty() {
            let nama_file = &nama_file;
            let (mut x, mut y) = (0, 1);
            'dua: loop {
                match &data[x..y] {
                    "\n" | "\r" | ";" => {
                        kirim
                            .send((no_baris, nama_file.clone(), ";".to_string()))
                            .unwrap();
                        continue 'main;
                    }
                    "\"" => {
                        for i in x + 1.. {
                            y += 1;
                            if &data[i..y] == "\"" {
                                kirim
                                    .send((no_baris, nama_file.clone(), data[x..y].to_string()))
                                    .unwrap();
                                x = i + 1;
                                y = x + 1;
                                continue 'dua;
                            }
                        }
                        /*
                        let awal = x.clone() ;
                        loop{
                            x += 1;
                            y += 1;
                            if &data[x..y] == "\"" {
                                kirim.send((no_baris,nama_file.clone(),data[awal..y].to_string())).unwrap();
                                //#[ignore(unsused_assignment)]
                                //x += 1;
                                //y += 1;
                                //continue 'main
                                x += 1;
                                y += 1;
                                if y < data.len() {
                                    kirim.send((no_baris,nama_file.clone(),";".to_string())).unwrap();
                                    continue 'dua
                                } else {
                                    continue 'main
                                }
                            } /*else if &data[x..y] == "\n" {
                                data.pop();
                                data.push_str(
                                    terima.recv().unwrap().2.as_str()
                                );
                            }*/
                        }
                        */ //panic!();
                    }
                    "-" => {
                        x += 1;
                        y = x + 1;
                        if &data[x..y] == ">" {
                            x += 1;
                            y = x + 1;
                            kirim
                                .send((no_baris, nama_file.clone(), "->".to_string()))
                                .unwrap();
                        } else {
                            kirim
                                .send((no_baris, nama_file.clone(), "-".to_string()))
                                .unwrap();
                        }
                        continue;
                    }
                    "," | ":" | "&" | "." | "?" | "[" | "]" | "{" | "}" => {
                        kirim
                            .send((no_baris, nama_file.clone(), data[x..y].to_string()))
                            .unwrap();
                    }
                    "=" | "<" | ">" | "!" | "+" | "/" | "*" | "%" => {
                        x += 1;
                        y = x + 1;
                        if &data[x..y] != "=" {
                            kirim
                                .send((no_baris, nama_file.clone(), data[x - 1..y - 1].to_string()))
                                .unwrap();
                            continue 'dua;
                        } else {
                            kirim
                                .send((no_baris, nama_file.clone(), data[x - 1..y].to_string()))
                                .unwrap();
                        }
                    }
                    d if d != " " => {
                        for mut i in x + 1.. {
                            match &data[i..i + 1] {
                                " " | ";" | "\"" | "\r" => {
                                    kirim
                                        .send((no_baris, nama_file.clone(), data[x..i].to_string()))
                                        .unwrap();
                                    x = i;
                                    y = x + 1;
                                    continue 'dua;
                                }
                                "." => {
                                    if data[x..i].parse::<f64>().is_ok() {
                                        loop {
                                            i += 1;
                                            if !data[x..i].parse::<f64>().is_ok() {
                                                kirim
                                                    .send((
                                                        no_baris,
                                                        nama_file.clone(),
                                                        data[x..i].to_string(),
                                                    ))
                                                    .unwrap();
                                                x = i - 1;
                                                y = x + 1;
                                                continue 'dua;
                                            }
                                        }
                                    } else {
                                        kirim
                                            .send((
                                                no_baris,
                                                nama_file.clone(),
                                                data[x..i].to_string(),
                                            ))
                                            .unwrap();
                                        kirim
                                            .send((no_baris, nama_file.clone(), ".".to_string()))
                                            .unwrap();
                                        x = i + 1;
                                        y = x + 1;
                                        continue 'dua;
                                    }
                                }
                                g if g == "<"
                                    || g == ">"
                                    || g == "="
                                    || g == ":"
                                    || g == "!"
                                    || g == ","
                                    || g == "&"
                                    || g == "*"
                                    || g == "("
                                    || g == ")"
                                    || g == "["
                                    || g == "]" =>
                                {
                                    kirim
                                        .send((no_baris, nama_file.clone(), data[x..i].to_string()))
                                        .unwrap();
                                    kirim
                                        .send((no_baris, nama_file.clone(), g.to_string()))
                                        .unwrap();
                                    x = i + 1;
                                    y = x + 1;
                                    continue 'dua;
                                }
                                "\n" => {
                                    kirim
                                        .send((no_baris, nama_file.clone(), data[x..i].to_string()))
                                        .unwrap();
                                    kirim
                                        .send((no_baris, nama_file.clone(), ";".to_string()))
                                        .unwrap();
                                    continue 'main;
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
                x += 1;
                y = x + 1;
                /* percobaan
                match &data[x..y] {
                    "," | ":" | "&" | "." | "?" | "[" | "]" | "{" | "}" => {
                        kirim.send(data[x..y].to_string()).unwrap();
                        x += 1;
                        y += 1;
                    }
                    "=" | "<" | ">" | "!" | "+" | "-" | "/" | "*" | "%" => {
                        x += 1;
                        y += 1;
                        if data[x..y].to_string() == "=" {
                            kirim.send(data[x..y].to_string()).unwrap();
                        } else {
                            kirim.send(data[x..1].to_string()).unwrap();
                        }
                        y = 1
                    }
                    "\n" | "\r" | ";" => {
                        kirim.send(";".to_string()).unwrap();
                        continue 'main;
                    }
                    " " => x += 1,
                    "\"" =>{
                        for i in x + 1 .. {
                            if &data[x..1] == "\""{
                                kirim.send(
                                    data[x..i].to_string()
                                ).unwrap();
                                x = i ;
                                continue 'dua;
                            }
                        }
                    }
                    "$" =>{
                        match &data[x..2]{
                            "$!"|"$?" =>{
                                kirim.send(data[x..x + 2].to_string()).unwrap();
                                x += 2
                            }
                            _=>{
                                kirim.send(data[x..x + 1].to_string()).unwrap();
                                x += 1
                            }
                        }
                    }
                    _ => {
                        for mut i in x + 1.. {
                            match &data[i..1] {
                                " " => {
                                    kirim.send(data[x..i].to_string()).unwrap();
                                    x = i;
                                    continue 'dua;
                                }
                                "<" | ">" | "=" | ":" | "!" | "," | "&" | "(" | ")" | "[" | "]"
                                | "+" | "-" | "/" | "*" | "%" => {
                                    kirim.send(data[x..i].to_string()).unwrap();
                                    kirim.send(data[i..1].to_string()).unwrap();
                                    x = i;
                                    continue 'dua;
                                }
                                /*
                                "\""=>{
                                    kirim.send(data[x..i].to_string()).unwrap();
                                    for i in i+1..{

                                    }
                                    continue 'main
                                }
                                */
                                /*
                                "<"|">"|"="|":"|"!"|","|"&"|"*"|"("|")" =>{
                                    x = i ;
                                    continue 'dua
                                }
                                */
                                "\n" | "\r" => {
                                    kirim.send(data[x..i].to_string()).unwrap();
                                    continue 'main;
                                }
                                "." => {
                                    if data[x..i + 1].parse::<f64>().is_ok() {
                                        i += 2;
                                        while data[x..i].parse::<f64>().is_ok() {i += 1}
                                        kirim.send(data[x..i].to_string()).unwrap();
                                        x = i;
                                        continue 'dua;
                                    } else {
                                        kirim.send(data[x..i].to_string()).unwrap();
                                        kirim.send(data[i..1].to_string()).unwrap();
                                        x = i;
                                        continue 'dua;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                */
            }
        } else {
            break;
        }
    }
    kirim.send((0, "".to_string(), "".to_string())).unwrap();
}
