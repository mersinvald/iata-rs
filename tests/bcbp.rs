extern crate iata;

use iata::bcbp::*;

#[test]
fn errors() {
    match BCBP::from("") {
        Ok(_)  => assert!(false),
        Err(e) => assert!(e == Error::DataLength),
    }

    match BCBP::from("X1BRUNER/ROMAN MR     EJNUFFX MUCSVOSU 2327 231L013A0052 100") {
        Ok(_)  => assert!(false),
        Err(e) => assert!(e == Error::FormatCode),
    }

    match BCBP::from("M0BRUNER/ROMAN MR     EJNUFFX MUCSVOSU 2327 231L013A0052 100") {
        Ok(_)  => assert!(false),
        Err(e) => assert!(e == Error::SegmentsCount)
    }

    match BCBP::from("MABRUNER/ROMAN MR     EJNUFFX MUCSVOSU 2327 231L013A0052 100") {
        Ok(_)  => assert!(false),
        Err(e) => assert!(e == Error::SegmentsCount)
    }

    match BCBP::from("M1BRUNER/ROMAN MR     EJNUFFX MUCSVOSU 2327 231L013A0052 1FF") {
        Ok(_)  => assert!(false),
        Err(e) => assert!(e == Error::CoditionalDataSize)
    }

        println!("{:?}", BCBP::from("M1BRUNER/ROMAN MR     EJNUFFX MUCSVOSU 2327 231L013A0052 100"));
        // assert!(false);

}

#[test]
fn mandatory1() {
    let src = "M1JOHN/SMITH JORDAN   EABCDEF JFKSVOSU 1234A001Y001Z0007 000";
    let tmp = BCBP::from(src);

    print!("RES {:?}", tmp);

    assert!(tmp.is_ok());

    let bcbp = tmp.unwrap();

    assert!(bcbp.name()        == "JOHN/SMITH JORDAN");
    assert!(bcbp.name_last()   == "JOHN");
    assert!(bcbp.name_first()  == "SMITH JORDAN");
    assert!(bcbp.ticket_flag()  == 'E');
    assert!(bcbp.segments[0].pnr() == "ABCDEF");
    assert!(bcbp.segments[0].src_airport()  == "JFK");
    assert!(bcbp.segments[0].dst_airport()  == "SVO");
    assert!(bcbp.segments[0].airline()      == "SU");
    assert!(bcbp.segments[0].flight_code()  == "1234A");
    assert!(bcbp.segments[0].flight_day()   == 1);
    assert!(bcbp.segments[0].flight_date(2017) == NaiveDate::from_ymd(2017, 1, 1));
    assert!(bcbp.segments[0].flight_day_aligned()   == "001");
    assert!(bcbp.segments[0].compartment()  == 'Y');
    assert!(bcbp.segments[0].seat()         == "1Z");
    assert!(bcbp.segments[0].seat_aligned() == "001Z");
    assert!(bcbp.segments[0].sequence()         == 7);
    assert!(bcbp.segments[0].sequence_aligned() == "0007");
    assert!(bcbp.segments[0].pax_status()   == "0");
    assert!(bcbp.build().unwrap() == src);
}

#[test]
fn mandatory4() {
    let src = "M4VERYLONGESTLASTNAMEDEABCDEF JFKSVOSU 1234 207          000ABCDEF SVOLEDSU 5678 210          000ABCDEF LEDSVOSU 9876 215          000ABCDEF SVOJFKSU 1357 215          000";
    let tmp = BCBP::from(src);

    assert!(tmp.is_ok());

    let bcbp = tmp.unwrap();

    assert!(bcbp.name()       == "VERYLONGESTLASTNAMED");
    assert!(bcbp.name_last()  == "VERYLONGESTLASTNAMED");
    assert!(bcbp.name_first() == "");
    assert!(bcbp.ticket_flag() == 'E');
    assert!(bcbp.segments[0].pnr()  == "ABCDEF");
    assert!(bcbp.segments[0].src_airport()  == "JFK");
    assert!(bcbp.segments[0].dst_airport()  == "SVO");
    assert!(bcbp.segments[0].airline()      == "SU");
    assert!(bcbp.segments[0].flight_code()  == "1234");
    assert!(bcbp.segments[0].flight_day()   == 207);
    assert!(bcbp.segments[1].pnr()  == "ABCDEF");
    assert!(bcbp.segments[1].src_airport()  == "SVO");
    assert!(bcbp.segments[1].dst_airport()  == "LED");
    assert!(bcbp.segments[1].airline()      == "SU");
    assert!(bcbp.segments[1].flight_code()  == "5678");
    assert!(bcbp.segments[1].flight_day()   == 210);
    assert!(bcbp.segments[2].pnr()  == "ABCDEF");
    assert!(bcbp.segments[2].src_airport()  == "LED");
    assert!(bcbp.segments[2].dst_airport()  == "SVO");
    assert!(bcbp.segments[2].airline()      == "SU");
    assert!(bcbp.segments[2].flight_code()  == "9876");
    assert!(bcbp.segments[2].flight_day()   == 215);
    assert!(bcbp.segments[3].pnr()  == "ABCDEF");
    assert!(bcbp.segments[3].src_airport()  == "SVO");
    assert!(bcbp.segments[3].dst_airport()  == "JFK");
    assert!(bcbp.segments[3].airline()      == "SU");
    assert!(bcbp.segments[3].flight_code()  == "1357");
    assert!(bcbp.segments[3].flight_day()   == 215);

    println!("BLD{:?}\nSRC{:?}", bcbp.build().unwrap(), src);

    assert!(bcbp.build().unwrap() == src);
}

#[test]
fn conditional3() {
    let src = "M3JOHN/SMITH          EABCDEF JFKSVOSK 1234 123M014C0050 35D>5180O 0276BSK              2A55559467513980 SK                         *30600000K09         ABCDEF SVOFRASU 5678 135Y013A0012 3372A55559467513990 SU SU 12345678             09         ABCDEF FRAJFKSU 9876 231Y022F0052 3372A55559467513990 SU SU 12345678             09         ";
    println!("|");
    let tmp = BCBP::from(src);
    println!("TMP {:?}", tmp);

    assert!(tmp.is_ok());

    let bcbp = tmp.unwrap();

    println!("{:?}", bcbp);

    assert!(bcbp.name()       == "JOHN/SMITH");
    assert!(bcbp.name_last()  == "JOHN");
    assert!(bcbp.name_first() == "SMITH");
    assert!(bcbp.ticket_flag() == 'E');
    assert!(bcbp.segments[0].pnr()  == "ABCDEF");
    assert!(bcbp.segments[0].src_airport()  == "JFK");
    assert!(bcbp.segments[0].dst_airport()  == "SVO");
    assert!(bcbp.segments[0].airline()      == "SK");
    assert!(bcbp.segments[0].flight_code()  == "1234");
    assert!(bcbp.segments[0].flight_day()   == 123);
    assert!(bcbp.segments[1].pnr()  == "ABCDEF");
    assert!(bcbp.segments[1].src_airport()  == "SVO");
    assert!(bcbp.segments[1].dst_airport()  == "FRA");
    assert!(bcbp.segments[1].airline()      == "SU");
    assert!(bcbp.segments[1].flight_code()  == "5678");
    assert!(bcbp.segments[1].flight_day()   == 135);
    assert!(bcbp.segments[2].pnr()  == "ABCDEF");
    assert!(bcbp.segments[2].src_airport()  == "FRA");
    assert!(bcbp.segments[2].dst_airport()  == "JFK");
    assert!(bcbp.segments[2].airline()      == "SU");
    assert!(bcbp.segments[2].flight_code()  == "9876");
    assert!(bcbp.segments[2].flight_day()   == 231);
}

