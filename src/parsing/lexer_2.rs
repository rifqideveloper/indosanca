pub fn baca(terima: std::sync::mpsc::Receiver<(/*nomer baris*/ u64, /*nama file*/ String,/*nilai*/ String)>, kirim: std::sync::mpsc::Sender<(u64,String,String)>) {
    // tokenizer 2.1.0
    //let mut _str = (false,false,false);
    //let mut blok = (false,0u64,0u64,0u64);
    //fn str_() {}
    'main: loop {
        let data = terima.recv().unwrap();
        if !data.2.is_empty() {
            let nama_file = data.1;
            let (mut x, mut y) = (0, 1);
            'dua: loop {
                match &data.2[x..y] {
                    "," | ":" | "&" | "." | "?" | "[" | "]" | "{" | "}" => {
                        kirim.send((data.0,nama_file.clone(),data.2[x..y].to_string())).unwrap();
                    }
                    "=" | "<" | ">" | "!" | "+" | "-" | "/" | "*" | "%" => {
                        x += 1;
                        y = x + 1;
                        if &data.2[x..y] != "=" {
                            kirim.send((data.0,nama_file.clone(),data.2[x - 1..y - 1].to_string())).unwrap();
                            continue 'dua;
                        } else {
                            kirim.send((data.0,nama_file.clone(),data.2[x - 1..y].to_string())).unwrap();
                        }
                    }
                    ";" =>{
                        kirim.send((data.0,nama_file.clone(),";".to_string())).unwrap();
                    }
                    d if d != " " => {
                        for mut i in x + 1.. {
                            match &data.2[i..i + 1] {
                                " " | ";" | "\"" | "\r" => {
                                    kirim.send((data.0,nama_file.clone(),data.2[x..i].to_string())).unwrap();
                                    x = i;
                                    y = x + 1;
                                    continue 'dua;
                                }
                                "." => {
                                    if data.2[x..i].parse::<f64>().is_ok() {
                                        loop {
                                            i += 1;
                                            if !data.2[x..i].parse::<f64>().is_ok() {
                                                kirim.send((data.0,nama_file.clone(),data.2[x..i].to_string())).unwrap();
                                                x = i - 1;
                                                y = x + 1;
                                                continue 'dua;
                                            }
                                        }
                                    } else {
                                        kirim.send((data.0,nama_file.clone(),data.2[x..i].to_string())).unwrap();
                                        kirim.send((data.0,nama_file.clone(),".".to_string())).unwrap();
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
                                    || g == "]"
                                    =>
                                {
                                    kirim.send((data.0,nama_file.clone(),data.2[x..i].to_string())).unwrap();
                                    kirim.send((data.0,nama_file.clone(),g.to_string())).unwrap();
                                    x = i + 1;
                                    y = x + 1;
                                    continue 'dua
                                }
                                "\n"=>{
                                    kirim.send((data.0,nama_file.clone(),data.2[x..i].to_string())).unwrap();
                                    kirim.send((data.0,nama_file.clone(),";".to_string())).unwrap();
                                    continue 'main
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
    kirim.send((0,"".to_string(),"".to_string())).unwrap();
}
