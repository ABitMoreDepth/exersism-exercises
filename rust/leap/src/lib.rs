pub fn is_leap_year(year: u64) -> bool {
    if year.checked_rem(4) == Some(0) {
        if year.checked_rem(100) == Some(0) {
            if year.checked_rem(400) == Some(0) {
                return true;
            }

            return false;
        }

        return true;
    }
    return false;
}
