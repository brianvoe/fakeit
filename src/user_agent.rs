use crate::data::computer;
use crate::datetime;
use crate::misc;

pub fn chrome() -> String {
    let rand_num = misc::random(531, 536) + misc::random(0, 2);
    format!(
        "Mozilla/5.0 ({}) AppleWebKit/{} (KHTML, like Gecko) Chrome/{}.0.{}.0 Mobile Safari/{}",
        random_platform(),
        rand_num,
        misc::random(36, 40),
        misc::random(800, 899),
        rand_num
    )
}

pub fn firefox() -> String {
    // @TODO should be 2006-02-01
    let date = format!(
        "{}-{}-{}",
        datetime::year(),
        datetime::month(),
        datetime::day()
    );
    let platform = match misc::random(1, 3) {
        1 => format!(
            "({}; en-US; rv:1.9.{}.20)",
            windows_platform_token(),
            misc::random(0, 3)
        ),
        2 => format!("({}; rv:{}.0)", linux_platform_token(), misc::random(5, 8)),
        _ => format!("({} rv:{}.0)", mac_platform_token(), misc::random(2, 7)),
    };
    format!(
        "Mozilla/5.0 {} Gecko/{} Firefox/{}.0",
        platform,
        date,
        misc::random(35, 37)
    )
}

pub fn safari() -> String {
    let rand_num = format!(
        "{}.{}.{}",
        misc::random(531, 536),
        misc::random(1, 51),
        misc::random(1, 8),
    );

    let ver = format!("{}.{}", misc::random(4, 6), misc::random(0, 2));

    let mobile_devices = match misc::random(1, 2) {
        1 => String::from("iPhone; CPU iPhone OS"),
        _ => String::from("iPad; CPU OS"),
    };

    let platforms = match misc::random(1,3) {
        1 => format!("(Windows; U; {}) AppleWebKit/{} (KHTML, like Gecko) Version/{} Safari/{}", windows_platform_token(), rand_num, ver, rand_num),
        2 => format!("({} rv:{}.0; en-US) AppleWebKit/{} (KHTML, like Gecko) Version/{} Safari/{}", mac_platform_token(), misc::random(4, 7), rand_num, ver, rand_num),
        _ => format!("({} {}_{}_{} like Mac OS X; en-US) AppleWebKit/{} (KHTML, like Gecko) Version/{}.0.5 Mobile/8B{} Safari/6{}", mobile_devices, misc::random(7, 9 ), misc::random(0, 3), misc::random(1, 3), rand_num, misc::random(3, 5), misc::random(111, 120), rand_num)
    };

    format!("Mozilla/5.0 {}", platforms)
}

pub fn opera() -> String {
    let platform = format!(
        "({}; en-US) Presto/2.{}.{} Version/{}.00",
        random_platform(),
        misc::random(8, 13),
        misc::random(160, 355),
        misc::random(10, 13)
    );

    format!(
        "Opera/{}.{} {}",
        misc::random(8, 10),
        misc::random(10, 99),
        platform
    )
}

pub fn linux_platform_token() -> String {
    format!(
        "X11; Linux {}",
        misc::random_data(computer::LINUX_PROCESSOR).to_string()
    )
}

pub fn mac_platform_token() -> String {
    format!(
        "Macintosh; {} Mac OS X 10_{}_{}",
        misc::random_data(computer::MAC_PROCESSOR).to_string(),
        misc::random(5, 9),
        misc::random(0, 10),
    )
}

pub fn windows_platform_token() -> String {
    misc::random_data(computer::WINDOWS_PLATFORM).to_string()
}

pub fn random_platform() -> String {
    match misc::random(1, 3) {
        1 => linux_platform_token(),
        2 => mac_platform_token(),
        _ => windows_platform_token(),
    }
}

#[cfg(test)]
mod tests {
    use crate::user_agent;
    use crate::test_helper;

    #[test]
    fn chrome() {
        let data1 = user_agent::chrome();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn firefox() {
        let data1 = user_agent::firefox();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn safari() {
        let data1 = user_agent::safari();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn opera() {
        let data1 = user_agent::opera();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn linux_platform_token() {
        let data1 = user_agent::linux_platform_token();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn mac_platform_token() {
        let data1 = user_agent::mac_platform_token();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn windows_platform_token() {
        let data1 = user_agent::windows_platform_token();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn random_platform() {
        let data1 = user_agent::random_platform();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }
}
