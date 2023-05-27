fn answer_bit(ns: i32, x: i32, y: i32) -> i32 {
    (ns >> ((x << 1) | y)) & 1
}

fn main() {
    println!("name: Task 3");
    println!("init: read_ns_1_0");
    println!("accept: final");

    for step in 0..4 {
        // Прочитаем изначальные n0, n1, n2, n3
        for ns in 0..1 << step {
            for x in 0..2 {
                println!("read_ns_{}_{ns},{x}", step + 1);
                println!("read_ns_{}_{},_,>", step + 2, ns | (x << step));
            }
        }
    }

    for ns in 0..16 {
        // Найдём конец числа A
        println!("read_ns_5_{ns},_");
        println!("seek_end_a_1_{ns},_,>");
    }

    for ns in 0..16 {
        // A == {}
        println!("seek_end_a_1_{ns},_");
        println!("fill_rest_b_1_{ns},_,>");

        // B == {}
        println!("fill_rest_b_1_{ns},_");
        println!("final,_,>");

        for x in 0..2 {
            // B != {}
            println!("fill_rest_b_1_{ns},{x}");
            println!("fill_rest_b_2_{ns}_{},_,>", answer_bit(ns, 0, x));

            for y in 0..2 {
                println!("fill_rest_b_2_{ns}_{x},{y}");
                println!("fill_rest_b_2_{ns}_{},{x},>", answer_bit(ns, 0, y));
            }

            println!("fill_rest_b_2_{ns}_{x},_");
            println!("seek_answer_start,{x},<");
        }
    }

    for x in 0..2 {
        println!("seek_answer_start,{x}");
        println!("seek_answer_start,{x},<");
    }
    println!("seek_answer_start,_");
    println!("final,_,>");

    for ns in 0..16 {
        // A != {}
        for x in 0..2 {
            // Сдвинем A вправо и запомним младший бит
            println!("seek_end_a_1_{ns},{x}");
            println!("seek_end_a_2_{ns}_{x},_,>");

            for y in 0..2 {
                println!("seek_end_a_2_{ns}_{x},{y}");
                println!("seek_end_a_2_{ns}_{y},{x},>");
            }

            // Сдвинули A, теперь ищем конец B
            println!("seek_end_a_2_{ns}_{x},_");
            println!("seek_end_b_1_{ns}_{x},_,>");

            // B == {}
            println!("seek_end_b_1_{ns}_{x},_");
            println!("write_and_back_0_{ns}_{},_,>", answer_bit(ns, x, 0));

            println!("write_and_back_0_{ns}_{x},_");
            println!("write_and_back_1_{ns}_{x},_,>");
        }
    }
}
