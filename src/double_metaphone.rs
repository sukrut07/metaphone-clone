//! Double Metaphone state machine and phonetic rule engine.

use crate::constants::VOWELS;
use crate::word::Word;

/// Action emitted by character rule handlers indicating phone token additions and buffer index step.
#[derive(Debug, Clone, Copy)]
pub enum NextAction {
    /// Same token added to both primary and secondary codes.
    Same(Option<&'static str>, usize),
    /// Different tokens added to primary and secondary codes.
    Diff(Option<&'static str>, Option<&'static str>, usize),
}

/// Double Metaphone algorithm parser state machine.
#[derive(Debug, Clone)]
pub struct DoubleMetaphone {
    /// Current character index position in `word.buffer`.
    pub position: usize,
    /// Accumulated primary phonetic code string.
    pub primary_phone: String,
    /// Accumulated secondary phonetic code string.
    pub secondary_phone: String,
    /// Next step action signal.
    pub next: NextAction,
    /// Preprocessed word buffer context.
    pub word: Word,
}

impl DoubleMetaphone {
    /// Constructs a new `DoubleMetaphone` instance for `input`.
    /// Pre-allocates string buffers with capacity 8 to avoid heap re-allocations.
    pub fn new(input: &str) -> Self {
        Self {
            position: 0,
            primary_phone: String::with_capacity(8),
            secondary_phone: String::with_capacity(8),
            next: NextAction::Same(None, 1),
            word: Word::new(input),
        }
    }

    /// Convenience wrapper to parse `input` directly into a `(primary, secondary)` tuple.
    pub fn parse(input: &str) -> (String, String) {
        let mut dm = DoubleMetaphone::new(input);
        dm.execute()
    }

    /// Runs the state machine algorithm loop over the word buffer.
    pub fn execute(&mut self) -> (String, String) {
        self.position = self.word.start_index;
        self.check_word_start();

        while self.position <= self.word.end_index {
            let character = self.word.at(self.position);
            self.next = NextAction::Same(None, 1);

            if VOWELS.contains(&character) {
                self.process_initial_vowels();
            } else {
                match character {
                    ' ' => {
                        self.position += 1;
                        continue;
                    }
                    'B' => self.process_b(),
                    'C' => self.process_c(),
                    'D' => self.process_d(),
                    'F' => self.process_f(),
                    'G' => self.process_g(),
                    'H' => self.process_h(),
                    'J' => self.process_j(),
                    'K' => self.process_k(),
                    'L' => self.process_l(),
                    'M' => self.process_m(),
                    'N' => self.process_n(),
                    'P' => self.process_p(),
                    'Q' => self.process_q(),
                    'R' => self.process_r(),
                    'S' => self.process_s(),
                    'T' => self.process_t(),
                    'V' => self.process_v(),
                    'W' => self.process_w(),
                    'X' => self.process_x(),
                    'Z' => self.process_z(),
                    _ => {
                        self.position += 1;
                        continue;
                    }
                }
            }

            self.apply_next();
        }

        // Avoid cloning Strings by taking ownership of the accumulated buffers.
        let primary = std::mem::take(&mut self.primary_phone);
        let secondary = if primary == self.secondary_phone {
            String::new()
        } else {
            std::mem::take(&mut self.secondary_phone)
        };

        (primary, secondary)
    }

    fn apply_next(&mut self) {
        match self.next {
            NextAction::Same(token, advance) => {
                if let Some(t) = token {
                    if !t.is_empty() {
                        self.primary_phone.push_str(t);
                        self.secondary_phone.push_str(t);
                    }
                }
                self.position += advance;
            }
            NextAction::Diff(prim, sec, advance) => {
                if let Some(p) = prim {
                    if !p.is_empty() {
                        self.primary_phone.push_str(p);
                    }
                }
                if let Some(s) = sec {
                    if !s.is_empty() {
                        self.secondary_phone.push_str(s);
                    }
                }
                self.position += advance;
            }
        }
    }

    fn check_word_start(&mut self) {
        let start = self.word.start_index;
        if self
            .word
            .in_strs_at(start, crate::constants::SILENT_STARTERS)
        {
            self.position += 1;
        }
        if self.word.is_at(self.position, 'X') {
            self.primary_phone.push('S');
            self.secondary_phone.push('S');
            self.position += 1;
        }
    }

    fn process_initial_vowels(&mut self) {
        self.next = NextAction::Same(None, 1);
        if self.position == self.word.start_index {
            self.next = NextAction::Same(Some("A"), 1);
        }
    }

    fn process_b(&mut self) {
        if self.word.is_at(self.position + 1, 'B') {
            self.next = NextAction::Same(Some("P"), 2);
        } else {
            self.next = NextAction::Same(Some("P"), 1);
        }
    }

    #[allow(clippy::if_same_then_else)]
    fn process_c(&mut self) {
        let pos = self.position;
        let start_index = self.word.start_index;
        let w = &self.word;

        if pos > start_index + 1
            && !w.is_vowel_at(pos - 2)
            && w.eq_at_back(pos, 1, "ACH")
            && w.at(pos + 2) != 'I'
            && (w.at(pos + 2) != 'E' || w.in_strs_at_back(pos, 2, &["BACHER", "MACHER"]))
        {
            self.next = NextAction::Same(Some("K"), 2);
        } else if pos == start_index && w.eq_at(start_index, "CAESAR") {
            self.next = NextAction::Same(Some("S"), 2);
        } else if w.eq_at(pos, "CHIA") {
            self.next = NextAction::Same(Some("K"), 2);
        } else if w.eq_at(pos, "CH") {
            if pos > start_index && w.eq_at(pos, "CHAE") {
                self.next = NextAction::Diff(Some("K"), Some("X"), 2);
            } else if pos == start_index
                && (w.in_strs_at(pos + 1, &["HARAC", "HARIS"])
                    || w.in_strs_at(pos + 1, &["HOR", "HYM", "HIA", "HEM"]))
                && !w.eq_at(start_index, "CHORE")
            {
                self.next = NextAction::Same(Some("K"), 2);
            } else if w.in_strs_at(start_index, &["VAN ", "VON "])
                || w.eq_at(start_index, "SCH")
                || w.in_strs_at_back(pos, 2, &["ORCHES", "ARCHIT", "ORCHID"])
                || w.in_at(pos + 2, &['T', 'S'])
                || ((w.in_at(pos - 1, &['A', 'O', 'U', 'E']) || pos == start_index)
                    && w.in_at(pos + 2, &['L', 'R', 'N', 'M', 'B', 'H', 'F', 'V', 'W', ' ']))
            {
                self.next = NextAction::Same(Some("K"), 2);
            } else {
                if pos > start_index {
                    if w.eq_at(start_index, "MC") {
                        self.next = NextAction::Same(Some("K"), 2);
                    } else {
                        self.next = NextAction::Diff(Some("X"), Some("K"), 2);
                    }
                } else {
                    self.next = NextAction::Same(Some("X"), 2);
                }
            }
        } else if w.eq_at(pos, "CZ") && !w.eq_at_back(pos, 2, "WICZ") {
            self.next = NextAction::Diff(Some("S"), Some("X"), 2);
        } else if w.eq_at(pos + 1, "CIA") {
            self.next = NextAction::Same(Some("X"), 3);
        } else if w.eq_at(pos, "CC") && !(pos == start_index + 1 && w.is_at(start_index, 'M')) {
            if w.in_at(pos + 2, &['I', 'E', 'H']) && !w.eq_at(pos + 2, "HU") {
                if (pos == start_index + 1 && w.is_at(start_index, 'A'))
                    || w.in_strs_at_back(pos, 1, &["UCCEE", "UCCES"])
                {
                    self.next = NextAction::Same(Some("KS"), 3);
                } else {
                    self.next = NextAction::Same(Some("X"), 3);
                }
            } else {
                self.next = NextAction::Same(Some("K"), 2);
            }
        } else if w.in_strs_at(pos, &["CK", "CG", "CQ"]) {
            self.next = NextAction::Same(Some("K"), 2);
        } else if w.in_strs_at(pos, &["CI", "CE", "CY"]) {
            if w.in_strs_at(pos, &["CIO", "CIE", "CIA"]) {
                self.next = NextAction::Diff(Some("S"), Some("X"), 2);
            } else {
                self.next = NextAction::Same(Some("S"), 2);
            }
        } else {
            if w.in_strs_at(pos + 1, &[" C", " Q", " G"]) {
                self.next = NextAction::Same(Some("K"), 3);
            } else {
                if w.in_at(pos + 1, &['C', 'K', 'Q']) && !w.in_strs_at(pos + 1, &["CE", "CI"]) {
                    self.next = NextAction::Same(Some("K"), 2);
                } else {
                    self.next = NextAction::Same(Some("K"), 1);
                }
            }
        }
    }

    fn process_d(&mut self) {
        let pos = self.position;
        let w = &self.word;
        if w.eq_at(pos, "DG") {
            if w.in_at(pos + 2, &['I', 'E', 'Y']) {
                self.next = NextAction::Same(Some("J"), 3);
            } else {
                self.next = NextAction::Same(Some("TK"), 2);
            }
        } else if w.in_strs_at(pos, &["DT", "DD"]) {
            self.next = NextAction::Same(Some("T"), 2);
        } else {
            self.next = NextAction::Same(Some("T"), 1);
        }
    }

    fn process_f(&mut self) {
        if self.word.is_at(self.position + 1, 'F') {
            self.next = NextAction::Same(Some("F"), 2);
        } else {
            self.next = NextAction::Same(Some("F"), 1);
        }
    }

    #[allow(clippy::if_same_then_else)]
    fn process_g(&mut self) {
        let pos = self.position;
        let start_index = self.word.start_index;
        let w = &self.word;

        if w.is_at(pos + 1, 'H') {
            if pos > start_index && !w.is_vowel_at(pos - 1) {
                self.next = NextAction::Same(Some("K"), 2);
            } else if pos < start_index + 3 {
                if pos == start_index {
                    if w.is_at(pos + 2, 'I') {
                        self.next = NextAction::Same(Some("J"), 2);
                    } else {
                        self.next = NextAction::Same(Some("K"), 2);
                    }
                }
            } else if (pos > start_index + 1 && w.in_at(pos.saturating_sub(2), &['B', 'H', 'D']))
                || (pos > start_index + 2 && w.in_at(pos.saturating_sub(3), &['B', 'H', 'D']))
                || (pos > start_index + 3 && w.in_at(pos.saturating_sub(4), &['B', 'H']))
            {
                self.next = NextAction::Same(None, 2);
            } else {
                if pos > start_index + 2
                    && w.is_at(pos - 1, 'U')
                    && w.in_at(pos.saturating_sub(3), &['C', 'G', 'L', 'R', 'T'])
                {
                    self.next = NextAction::Same(Some("F"), 2);
                } else {
                    if pos > start_index && !w.is_at(pos - 1, 'I') {
                        self.next = NextAction::Same(Some("K"), 2);
                    }
                }
            }
        } else if w.is_at(pos + 1, 'N') {
            if pos == start_index + 1 && w.is_vowel_at(start_index) && !w.is_slavo_germanic() {
                self.next = NextAction::Diff(Some("KN"), Some("N"), 2);
            } else {
                if !w.eq_at(pos + 2, "EY") && !w.is_at(pos + 1, 'Y') && !w.is_slavo_germanic() {
                    self.next = NextAction::Diff(Some("N"), Some("KN"), 2);
                } else {
                    self.next = NextAction::Same(Some("KN"), 2);
                }
            }
        } else if w.eq_at(pos + 1, "LI") && !w.is_slavo_germanic() {
            self.next = NextAction::Diff(Some("KL"), Some("L"), 2);
        } else if pos == start_index
            && (w.is_at(pos + 1, 'Y')
                || w.in_strs_at(
                    pos + 1,
                    &[
                        "ES", "EP", "EB", "EL", "EY", "IB", "IL", "IN", "IE", "EI", "ER",
                    ],
                ))
        {
            self.next = NextAction::Diff(Some("K"), Some("J"), 2);
        } else if (w.eq_at(pos + 1, "ER") || w.is_at(pos + 1, 'Y'))
            && !w.in_strs_at(start_index, &["DANGER", "RANGER", "MANGER"])
            && !w.in_at(pos - 1, &['E', 'I'])
            && !w.in_strs_at_back(pos, 1, &["RGY", "OGY"])
        {
            self.next = NextAction::Diff(Some("K"), Some("J"), 2);
        } else if w.in_at(pos + 1, &['E', 'I', 'Y']) || w.in_strs_at_back(pos, 1, &["AGGI", "OGGI"])
        {
            if w.in_strs_at(start_index, &["VON ", "VAN "])
                || w.eq_at(start_index, "SCH")
                || w.eq_at(pos + 1, "ET")
            {
                self.next = NextAction::Same(Some("K"), 2);
            } else {
                if w.eq_at(pos + 1, "IER ") {
                    self.next = NextAction::Same(Some("J"), 2);
                } else {
                    self.next = NextAction::Diff(Some("J"), Some("K"), 2);
                }
            }
        } else if w.is_at(pos + 1, 'G') {
            self.next = NextAction::Same(Some("K"), 2);
        } else {
            self.next = NextAction::Same(Some("K"), 1);
        }
    }

    fn process_h(&mut self) {
        let pos = self.position;
        let start_index = self.word.start_index;
        let w = &self.word;

        if (pos == start_index || w.is_vowel_at(pos - 1)) && w.is_vowel_at(pos + 1) {
            self.next = NextAction::Same(Some("H"), 2);
        } else {
            self.next = NextAction::Same(None, 1);
        }
    }

    fn process_j(&mut self) {
        let pos = self.position;
        let start_index = self.word.start_index;
        let end_index = self.word.end_index;
        let w = &self.word;

        let advance = if w.is_at(pos + 1, 'J') { 2 } else { 1 };

        if w.eq_at(pos, "JOSE") || w.eq_at(start_index, "SAN ") {
            if (pos == start_index && w.is_at(pos + 4, ' ')) || w.eq_at(start_index, "SAN ") {
                self.next = NextAction::Same(Some("H"), advance);
            } else {
                self.next = NextAction::Diff(Some("J"), Some("H"), advance);
            }
        } else if pos == start_index && !w.eq_at(pos, "JOSE") {
            self.next = NextAction::Diff(Some("J"), Some("A"), advance);
        } else {
            if w.is_vowel_at(pos - 1) && !w.is_slavo_germanic() && w.in_at(pos + 1, &['A', 'O']) {
                self.next = NextAction::Diff(Some("J"), Some("H"), advance);
            } else if pos == end_index {
                self.next = NextAction::Diff(Some("J"), Some(" "), advance);
            } else if !w.in_at(pos + 1, &['L', 'T', 'K', 'S', 'N', 'M', 'B', 'Z'])
                && !w.in_at(pos - 1, &['S', 'K', 'L'])
            {
                self.next = NextAction::Same(Some("J"), advance);
            } else {
                self.next = NextAction::Same(None, advance);
            }
        }
    }

    fn process_k(&mut self) {
        if self.word.is_at(self.position + 1, 'K') {
            self.next = NextAction::Same(Some("K"), 2);
        } else {
            self.next = NextAction::Same(Some("K"), 1);
        }
    }

    fn process_l(&mut self) {
        let pos = self.position;
        let end_index = self.word.end_index;
        let w = &self.word;

        if w.is_at(pos + 1, 'L') {
            if (pos + 2 == end_index && w.in_strs_at_back(pos, 1, &["ILLO", "ILLA", "ALLE"]))
                || ((w.in_strs_at_back(end_index, 1, &["AS", "OS"])
                    || w.in_at(end_index, &['A', 'O']))
                    && w.eq_at_back(pos, 1, "ALLE"))
            {
                self.next = NextAction::Diff(Some("L"), Some(""), 2);
            } else {
                self.next = NextAction::Same(Some("L"), 2);
            }
        } else {
            self.next = NextAction::Same(Some("L"), 1);
        }
    }

    fn process_m(&mut self) {
        let pos = self.position;
        let w = &self.word;

        if (w.eq_at(pos + 1, "UMB") && (pos + 1 == self.word.end_index || w.eq_at(pos + 2, "ER")))
            || w.is_at(pos + 1, 'M')
        {
            self.next = NextAction::Same(Some("M"), 2);
        } else {
            self.next = NextAction::Same(Some("M"), 1);
        }
    }

    fn process_n(&mut self) {
        if self.word.is_at(self.position + 1, 'N') {
            self.next = NextAction::Same(Some("N"), 2);
        } else {
            self.next = NextAction::Same(Some("N"), 1);
        }
    }

    fn process_p(&mut self) {
        let pos = self.position;
        let w = &self.word;
        if w.is_at(pos + 1, 'H') {
            self.next = NextAction::Same(Some("F"), 2);
        } else if w.in_at(pos + 1, &['P', 'B']) {
            self.next = NextAction::Same(Some("P"), 2);
        } else {
            self.next = NextAction::Same(Some("P"), 1);
        }
    }

    fn process_q(&mut self) {
        if self.word.is_at(self.position + 1, 'Q') {
            self.next = NextAction::Same(Some("K"), 2);
        } else {
            self.next = NextAction::Same(Some("K"), 1);
        }
    }

    fn process_r(&mut self) {
        let pos = self.position;
        let end_index = self.word.end_index;
        let w = &self.word;

        let advance = if w.is_at(pos + 1, 'R') { 2 } else { 1 };

        if pos == end_index
            && !w.is_slavo_germanic()
            && w.eq_at_back(pos, 2, "IE")
            && !w.in_strs_at_back(pos, 4, &["ME", "MA"])
        {
            self.next = NextAction::Diff(Some(""), Some("R"), advance);
        } else {
            self.next = NextAction::Same(Some("R"), advance);
        }
    }

    fn process_s(&mut self) {
        let pos = self.position;
        let start_index = self.word.start_index;
        let end_index = self.word.end_index;
        let w = &self.word;

        if w.in_strs_at_back(pos, 1, &["ISL", "YSL"]) {
            self.next = NextAction::Same(None, 1);
        } else if pos == start_index && w.eq_at(start_index, "SUGAR") {
            self.next = NextAction::Diff(Some("X"), Some("S"), 1);
        } else if w.eq_at(pos, "SH") {
            if w.in_strs_at(pos + 1, &["HEIM", "HOEK", "HOLM", "HOLZ"]) {
                self.next = NextAction::Same(Some("S"), 2);
            } else {
                self.next = NextAction::Same(Some("X"), 2);
            }
        } else if w.in_strs_at(pos, &["SIO", "SIA"]) || w.eq_at(pos, "SIAN") {
            if !w.is_slavo_germanic() {
                self.next = NextAction::Diff(Some("S"), Some("X"), 3);
            } else {
                self.next = NextAction::Same(Some("S"), 3);
            }
        } else if (pos == start_index && w.in_at(pos + 1, &['M', 'N', 'L', 'W']))
            || w.is_at(pos + 1, 'Z')
        {
            let advance = if w.is_at(pos + 1, 'Z') { 2 } else { 1 };
            self.next = NextAction::Diff(Some("S"), Some("X"), advance);
        } else if w.eq_at(pos, "SC") {
            if w.is_at(pos + 2, 'H') {
                if w.in_strs_at(pos + 3, &["OO", "ER", "EN", "UY", "ED", "EM"]) {
                    if w.in_strs_at(pos + 3, &["ER", "EN"]) {
                        self.next = NextAction::Diff(Some("X"), Some("SK"), 3);
                    } else {
                        self.next = NextAction::Same(Some("SK"), 3);
                    }
                } else {
                    if pos == start_index
                        && !w.is_vowel_at(start_index + 3)
                        && !w.is_at(start_index + 3, 'W')
                    {
                        self.next = NextAction::Diff(Some("X"), Some("S"), 3);
                    } else {
                        self.next = NextAction::Same(Some("X"), 3);
                    }
                }
            } else if w.in_at(pos + 2, &['I', 'E', 'Y']) {
                self.next = NextAction::Same(Some("S"), 3);
            } else {
                self.next = NextAction::Same(Some("SK"), 3);
            }
        } else if pos == end_index && w.in_strs_at_back(pos, 2, &["AI", "OI"]) {
            self.next = NextAction::Diff(Some(""), Some("S"), 1);
        } else {
            let advance = if w.in_at(pos + 1, &['S', 'Z']) { 2 } else { 1 };
            self.next = NextAction::Same(Some("S"), advance);
        }
    }

    #[allow(clippy::if_same_then_else)]
    fn process_t(&mut self) {
        let pos = self.position;
        let start_index = self.word.start_index;
        let w = &self.word;

        if w.eq_at(pos, "TION") || w.in_strs_at(pos, &["TIA", "TCH"]) {
            self.next = NextAction::Same(Some("X"), 3);
        } else if w.eq_at(pos, "TH") || w.eq_at(pos, "TTH") {
            if w.in_strs_at(pos + 2, &["OM", "AM"])
                || w.in_strs_at(start_index, &["VON ", "VAN "])
                || w.eq_at(start_index, "SCH")
            {
                self.next = NextAction::Same(Some("T"), 2);
            } else {
                self.next = NextAction::Diff(Some("0"), Some("T"), 2);
            }
        } else if w.in_at(pos + 1, &['T', 'D']) {
            self.next = NextAction::Same(Some("T"), 2);
        } else {
            self.next = NextAction::Same(Some("T"), 1);
        }
    }

    fn process_v(&mut self) {
        if self.word.is_at(self.position + 1, 'V') {
            self.next = NextAction::Same(Some("F"), 2);
        } else {
            self.next = NextAction::Same(Some("F"), 1);
        }
    }

    fn process_w(&mut self) {
        let pos = self.position;
        let start_index = self.word.start_index;
        let end_index = self.word.end_index;
        let w = &self.word;

        if w.eq_at(pos, "WR") {
            self.next = NextAction::Same(Some("R"), 2);
        } else if pos == start_index && (w.is_vowel_at(pos + 1) || w.eq_at(pos, "WH")) {
            if w.is_vowel_at(pos + 1) {
                self.next = NextAction::Diff(Some("A"), Some("F"), 1);
            } else {
                self.next = NextAction::Same(Some("A"), 1);
            }
        } else if (pos == end_index && w.is_vowel_at(pos - 1))
            || w.in_strs_at_back(pos, 1, &["EWSKI", "EWSKY", "OWSKI", "OWSKY"])
            || w.eq_at(start_index, "SCH")
        {
            self.next = NextAction::Diff(Some(""), Some("F"), 1);
        } else if w.in_strs_at(pos, &["WICZ", "WITZ"]) {
            self.next = NextAction::Diff(Some("TS"), Some("FX"), 4);
        } else {
            self.next = NextAction::Same(None, 1);
        }
    }

    fn process_x(&mut self) {
        let pos = self.position;
        let end_index = self.word.end_index;
        let w = &self.word;

        let advance = if w.in_at(pos + 1, &['C', 'X']) { 2 } else { 1 };

        let token = if pos == end_index
            && (w.in_strs_at_back(pos, 3, &["IAU", "EAU"])
                || w.in_strs_at_back(pos, 2, &["AU", "OU"]))
        {
            None
        } else {
            Some("KS")
        };

        self.next = NextAction::Same(token, advance);
    }

    fn process_z(&mut self) {
        let pos = self.position;
        let start_index = self.word.start_index;
        let w = &self.word;

        let advance = if w.in_at(pos + 1, &['Z', 'H']) { 2 } else { 1 };

        if w.is_at(pos + 1, 'H') {
            self.next = NextAction::Same(Some("J"), advance);
        } else if w.in_strs_at(pos + 1, &["ZO", "ZI", "ZA"])
            || (w.is_slavo_germanic() && pos > start_index && !w.is_at(pos.saturating_sub(1), 'T'))
        {
            self.next = NextAction::Diff(Some("S"), Some("TS"), advance);
        } else {
            self.next = NextAction::Same(Some("S"), advance);
        }
    }
}
