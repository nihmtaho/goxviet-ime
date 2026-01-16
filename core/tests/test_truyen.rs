use goxviet_core::*;

#[test]
fn test_truyen_transform() {
    ime_init();
    ime_method(0); // Telex

    // Type "truyeefn" should produce "truyền"
    let input = "truyeefn";
    let expected = "truyền";

    let mut output = String::new();

    for ch in input.chars() {
        let key = match ch {
            't' => 17,
            'r' => 15,
            'u' => 32,
            'y' => 16,
            'e' => 14,
            'f' => 3,
            'n' => 45,
            _ => continue,
        };

        let result = unsafe { ime_key_ext(key, false, false, false) };
        if !result.is_null() {
            let r_ref = unsafe { &*result };

            if r_ref.action == 1 && r_ref.count > 0 && !r_ref.chars.is_null() {
                output.clear();
                for i in 0..r_ref.count as usize {
                    let code = unsafe { *r_ref.chars.offset(i as isize) };
                    if let Some(c) = char::from_u32(code) {
                        output.push(c);
                    }
                }
            }

            unsafe { ime_free(result) };
        }
    }

    println!("Input: {}", input);
    println!("Output: {}", output);
    println!("Expected: {}", expected);

    assert_eq!(output, expected, "truyeefn should transform to truyền");
}
