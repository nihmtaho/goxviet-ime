#!/usr/bin/env python3
"""
Script to thoroughly clean Vietnamese dictionary by removing:
1. English words and loan words
2. Words with numbers
3. Words with special characters (especially !)
4. Chemical symbols and formulas
5. Technical terms
"""

import re
from pathlib import Path

# Comprehensive list of English words and loan words to remove
# Including French loan words (tiếng Pháp) commonly used in Vietnamese
LOAN_WORDS = {
    # Basic English words
    "abc",
    "ABC",
    "album",
    "algorithm",
    "almanac",
    "alpha",
    "alphabet",
    "amen",
    "ampere",
    "anbom",
    "anbum",
    "anbumin",
    "anmanac",
    "apphe",
    "apphich",
    "artel",
    "asphalt",
    "aspirin",
    "atlas",
    "automat",
    # Chemical elements and formulas
    "acetylen",
    "acetone",
    "aceton",
    "acmonica",
    "acmonica",
    "acquy",
    "acre",
    "aden",
    "adrenalin",
    "ag",
    "Ag",
    "al",
    "Al",
    "albumin",
    "alcaloid",
    "aldehyd",
    "alumin",
    "amian",
    "amib",
    "amid",
    "amid",
    "amiđan",
    "aminoacid",
    "amip",
    "ammoniac",
    "ampe",
    "ampere",
    "ampli",
    "amygdal",
    "anand",
    "ancaloit",
    "andehit",
    "anđehit",
    "angorit",
    "anod",
    "anofen",
    "anot",
    "anpha",
    "anten",
    "anthracit",
    "antimon",
    "antipirin",
    "antraxit",
    "antre",
    "antraxit",
    "apacthai",
    "apatit",
    "apxe",
    "ar",
    "Ar",
    "arsenic",
    "as",
    "As",
    "asen",
    "aten",
    "atropin",
    "au",
    "Au",
    "axetilen",
    "axeton",
    "axit",
    "azot",
    # Chemical acids
    "acid",
    "acetic",
    "amin",
    "carbonic",
    "chlorhydric",
    "clohidric",
    "nitric",
    "sulfuric",
    "sunfuric",
    "sunhidric",
    "axetic",
    "axit",
    "clohiđric",
    "clohydric",
    # Medical/technical
    "aden",
    "adrenalin",
    "ADN",
    "aids",
    "AIDS",
    "AK",
    "algol",
    "ALGOL",
    # Organizations and abbreviations
    "asean",
    "ASEAN",
    "ANZUS",
    "ATK",
    # Other foreign words
    "ad",
    "hoc",
    "posteriori",
    "priori",
    # Numbers written as words
    "ri",
    "e",
    "two",
    "one",
    # Common loan words that should be removed
    "anbom",
    "anbum",
    "anbumin",
    "ancaloit",
    "andehit",
    "angorit",
    "anmanac",
    "anod",
    "anofen",
    "anot",
    "anpha",
    "anten",
    "apacthai",
    "apatit",
    "apphe",
    "apphich",
    "apxe",
    "arbit",
    "armonica",
    "arsenic",
    "artel",
    "asen",
    "asphalt",
    "aspirin",
    "atlas",
    "atmosphe",
    "atropin",
    "automat",
    "axetilen",
    "axeton",
    "axit",
    "azot",
    # French loan words (từ mượn tiếng Pháp)
    "alô",
    "êtô",
    "ôtô",
    "ôkê",
    "balet",
    "balê",
    "bancông",
    "baxơ",
    "becgiê",
    "beton",
    "bêtông",
    "biđông",
    "biptêt",
    "bitcôt",
    "blốc",
    "bombê",
    "bon",
    "bôđê",
    "bonsêvich",
    "brôm",
    "buji",
    "bulông",
    "bupbê",
    "bupphê",
    "ca",
    "cab",
    "Caban",
    "Cac",
    "Cactông",
    "cafê",
    "calô",
    "camnhông",
    "canô",
    "canông",
    "catalô",
    "catôt",
    "catsê",
    "champua",
    "comlê",
    "commăng",
    "con",
    "côngtắc",
    "côngtenơ",
    "côngxectô",
    "côngxon",
    "côsin",
    "côtông",
    "cơlê",
    "crêp",
    "crếp",
    "crôm",
    "cuarơ",
    "culông",
    "dêrô",
    "derô",
    "đôminô",
    "duyra",
    "ec",
    "Ecca",
    "eczan",
    "ef",
    "ep",
    "etxăng",
    "fizê",
    "gab",
    "Galăng",
    "gara",
    "garô",
    "gatô",
    "gay",
    "ghiđông",
    "giămbông",
    "gilê",
    "glôcôm",
    "gôrila",
    "Hrê",
    "huơ",
    "Kủo",
    "ladơn",
    "lanhtô",
    "lavabô",
    "layơn",
    "lăclê",
    "lintô",
    "lôcôt",
    "macô",
    "manơcanh",
    "mayô",
    "măngđôlin",
    "măngsông",
    "môngtagiơ",
    "mộngtriệu",
    "nơron",
    "nơtron",
    "õbăng",
    "ôpêra",
    "ôplêt",
    "ôtôbuýt",
    "ôtôca",
    "ôtômat",
    "ôtôray",
    "palăng",
    "patê",
    "patinê",
    "phidê",
    "phôtô",
    "phrăng",
    "plăng",
    "plây",
    "quỵp",
    "quýu",
    "riđô",
    "rivê",
    "rônêô",
    "rôngđô",
    "rulô",
    "sêếu",
    "sôcôla",
    "sôpphơ",
    "suplơ",
    "tarô",
    "tatăng",
    "têtanôt",
    "tichkê",
    "tigôn",
    "tipô",
    "tôngđơ",
    "tônô",
    "tơrơt",
    "tơrưng",
    "tulơkhơ",
    "uở",
    "vagông",
    "varơi",
    "vettông",
    "vinilông",
    "viôlông",
    "viôlôngxen",
    "xalông",
    "xamôva",
    "xenlô",
    "xêri",
    "xichlô",
    "xifông",
    "xinê",
    "xirô",
    "xôviêt",
    "xtốp",
    "xtrết",
    "xuchiêng",
    "yôga",
}

# Words containing these patterns are likely English/loan words
ENGLISH_PATTERNS = [
    r"^[a-zA-Z]{1,4}$",  # Short all-caps or all-english words like ABC, ALGOL
    r".*[bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ]{3,}.*",  # 3+ consonants in a row
    r"^[^áàảãạâầấẩẫậăằắẳẵặéèẻẽẹêềếểễệíìỉĩịóòỏõọôồốổỗộơờớởỡợúùủũụưừứửữựýỳỷỹỵđĐ]+$",  # No Vietnamese diacritics
]


def contains_number(word):
    """Check if word contains any number."""
    return any(char.isdigit() for char in word)


def contains_exclamation(word):
    """Check if word contains exclamation mark."""
    return "!" in word


def contains_special_char(word):
    """Check if word contains special characters."""
    # Keep only Vietnamese letters and spaces
    allowed = set(
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZáàảãạâầấẩẫậăằắẳẵặéèẻẽẹêềếểễệíìỉĩịóòỏõọôồốổỗộơờớởỡợúùủũụưừứửữựýỳỷỹỵđĐ "
    )
    return any(char not in allowed for char in word)


def is_likely_english(word):
    """Check if word is likely an English/foreign word."""
    word_lower = word.lower().strip()

    # Direct match in loan word list
    if word_lower in LOAN_WORDS or word in LOAN_WORDS:
        return True

    # Check patterns
    for pattern in ENGLISH_PATTERNS:
        if re.match(pattern, word):
            return True

    # Check for too many consonants (Vietnamese has limited consonant clusters)
    consonants = "bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZ"
    consonant_count = sum(1 for c in word if c in consonants)
    if consonant_count > len(word) * 0.6:  # More than 60% consonants
        return True

    return False


def main():
    input_file = Path("core/tests/data/vietnamese_22k_pure.txt")
    pure_output = Path("core/tests/data/vietnamese_22k_pure.txt")
    loan_output = Path("core/tests/data/vietnamese_loan.txt")

    # Statistics
    stats = {
        "total": 0,
        "pure": 0,
        "english_loan": 0,
        "number": 0,
        "exclamation": 0,
        "special": 0,
    }

    pure_words = []
    loan_words = []

    print("Processing dictionary...")

    with open(input_file, "r", encoding="utf-8") as f:
        for line in f:
            word = line.strip()
            if not word:
                continue

            stats["total"] += 1

            # Check for exclamation mark (highest priority to remove)
            if contains_exclamation(word):
                stats["exclamation"] += 1
                loan_words.append((word, "contains_exclamation"))
                continue

            # Check for numbers
            if contains_number(word):
                stats["number"] += 1
                loan_words.append((word, "contains_number"))
                continue

            # Check for special characters
            if contains_special_char(word):
                stats["special"] += 1
                loan_words.append((word, "contains_special_char"))
                continue

            # Check if it's an English/loan word
            if is_likely_english(word):
                stats["english_loan"] += 1
                loan_words.append((word, "english_loan"))
                continue

            # Keep as pure Vietnamese
            stats["pure"] += 1
            pure_words.append(word)

    # Write pure Vietnamese words
    print(f"\nWriting {len(pure_words)} pure Vietnamese words...")
    with open(pure_output, "w", encoding="utf-8") as f:
        for word in pure_words:
            f.write(word + "\n")

    # Write loan words with category
    print(f"Writing {len(loan_words)} loan/invalid words...")
    with open(loan_output, "w", encoding="utf-8") as f:
        f.write("# Vietnamese Loan Words and Invalid Entries\n")
        f.write("# Format: word | category\n")
        f.write(
            "# Categories: english_loan, contains_number, contains_exclamation, contains_special_char\n\n"
        )
        for word, category in loan_words:
            f.write(f"{word} | {category}\n")

    # Print statistics
    print("\n" + "=" * 60)
    print("PROCESSING COMPLETE")
    print("=" * 60)
    print(f"Total words processed: {stats['total']}")
    print(
        f"Pure Vietnamese words: {stats['pure']} ({stats['pure'] / stats['total'] * 100:.1f}%)"
    )
    print(f"English/Loan words: {stats['english_loan']}")
    print(f"Words with numbers: {stats['number']}")
    print(f"Words with exclamation: {stats['exclamation']}")
    print(f"Words with special chars: {stats['special']}")
    print(
        f"Total removed: {len(loan_words)} ({len(loan_words) / stats['total'] * 100:.1f}%)"
    )
    print("\nOutput files:")
    print(f"  - {pure_output} ({len(pure_words)} words)")
    print(f"  - {loan_output} ({len(loan_words)} words)")


if __name__ == "__main__":
    main()
