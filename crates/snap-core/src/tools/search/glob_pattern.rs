//! `glob` 도구가 쓰는 이름 패턴 매칭 알고리즘 자리입니다(03-6 R3 SRP 분리).
//!
//! 도구 구현(`glob.rs`)과 패턴 해석은 바뀌는 이유가 달라 따로 둡니다(dev-principles 2절 SRP).

/// `*`·`?`·`**`만 지원하는 작은 이름 패턴 매처입니다.
///
/// 경로를 `/` 구분 상대 문자열로 보고, `*`는 구분자를 넘지 않습니다.
pub(crate) fn match_pattern(pattern: &str, text: &str) -> bool {
    let p: Vec<char> = pattern.chars().collect();
    let t: Vec<char> = text.chars().collect();
    let n = p.len();
    let m = t.len();
    let mut dp = vec![vec![false; m + 1]; n + 1];
    dp[n][m] = true;
    for i in (0..n).rev() {
        for j in (0..=m).rev() {
            dp[i][j] = match p[i] {
                '*' => {
                    if i + 1 < n && p[i + 1] == '*' {
                        let after = if i + 2 < n && p[i + 2] == '/' {
                            i + 3
                        } else {
                            i + 2
                        };
                        dp[after][j] || (j < m && dp[i][j + 1])
                    } else {
                        let mut hit = dp[i + 1][j];
                        let mut k = j;
                        while !hit && k < m && t[k] != '/' {
                            k += 1;
                            hit = dp[i + 1][k];
                        }
                        hit
                    }
                }
                '?' => j < m && t[j] != '/' && dp[i + 1][j + 1],
                other => j < m && char_eq(other, t[j]) && dp[i + 1][j + 1],
            };
        }
    }
    dp[0][0]
}

/// 글자 비교. Windows는 대소문자를 가리지 않고, 그 밖의 OS는 가립니다.
fn char_eq(a: char, b: char) -> bool {
    if cfg!(windows) {
        a.eq_ignore_ascii_case(&b)
    } else {
        a == b
    }
}
