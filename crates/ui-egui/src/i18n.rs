//! Lightweight UI localization for WordCraft.
//! Hebrew is intentionally kept in one module so adding more languages later is straightforward.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    English,
    Hebrew,
}

impl Language {
    pub const fn is_rtl(self) -> bool {
        matches!(self, Self::Hebrew)
    }
    pub const fn code(self) -> &'static str {
        match self { Self::English => "en", Self::Hebrew => "he" }
    }
}

pub fn tr(lang: Language, s: &str) -> String {
    if lang == Language::English { return s.to_owned(); }
    match s {
        "File" => "קובץ", "Home" => "בית", "Insert" => "הוספה", "Draw" => "ציור",
        "Design" => "עיצוב", "Layout" => "פריסה", "References" => "הפניות",
        "Mailings" => "דיוור", "Review" => "סקירה", "View" => "תצוגה", "Help" => "עזרה",
        "Table Design" => "עיצוב טבלה", "Table Layout" => "פריסת טבלה",
        "Clipboard" => "לוח", "Font" => "גופן", "Paragraph" => "פסקה",
        "Styles" => "סגנונות", "Editing" => "עריכה", "Reviewing" => "סקירה",
        "Share" => "שיתוף", "Comments" => "הערות", "Paste" => "הדבק",
        "Keep Text Only" => "שמור טקסט בלבד", "Merge Formatting" => "מזג עיצוב",
        "Cut" => "גזור", "Copy" => "העתק", "Format" => "עיצוב",
        "Format Painter" => "מברשת עיצוב", "Increase Font Size" => "הגדל גופן",
        "Decrease Font Size" => "הקטן גופן", "Change Case" => "שנה רישיות",
        "Sentence case." => "רישיות משפט.", "lowercase" => "אותיות קטנות",
        "UPPERCASE" => "אותיות גדולות", "Bold" => "מודגש", "Italic" => "נטוי",
        "Underline" => "קו תחתון", "Strikethrough" => "קו חוצה", "Text Highlight Color" => "צבע סימון",
        "Font Color" => "צבע גופן", "Align Left" => "יישור לשמאל", "Align Center" => "מרכוז",
        "Align Right" => "יישור לימין", "Justify" => "יישור לשני הצדדים",
        "Bullets" => "תבליטים", "Numbering" => "מספור", "Decrease Indent" => "הקטן כניסה",
        "Increase Indent" => "הגדל כניסה", "Line Spacing" => "מרווח שורות",
        "Borders" => "גבולות", "Clear Formatting" => "נקה עיצוב",
        "Themes" => "ערכות נושא", "Colors" => "צבעים", "Fonts" => "גופנים",
        "Watermark" => "סימן מים", "Page Color" => "צבע עמוד", "Page Borders" => "גבולות עמוד",
        "Margins" => "שוליים", "Orientation" => "כיוון", "Size" => "גודל",
        "Columns" => "עמודות", "Breaks" => "מעברים", "Indent" => "כניסה",
        "Spacing" => "מרווח", "Table" => "טבלה", "Picture" => "תמונה",
        "Shapes" => "צורות", "Icons" => "סמלים", "Link" => "קישור",
        "Header" => "כותרת עליונה", "Footer" => "כותרת תחתונה", "Page Number" => "מספר עמוד",
        "Text Box" => "תיבת טקסט", "Symbol" => "סמל", "Equation" => "משוואה",
        "Undo" => "בטל", "Redo" => "בצע שוב", "Save" => "שמור",
        "Print" => "הדפס", "Export" => "ייצוא", "Open" => "פתח",
        "New" => "חדש", "Close" => "סגור", "Options" => "אפשרויות",
        "Search commands and help" => "חיפוש פקודות ועזרה", "AutoSave" => "שמירה אוטומטית",
        "Focus" => "מיקוד", "Zoom In" => "הגדל תצוגה", "Zoom Out" => "הקטן תצוגה",
        "Track Changes: On" => "מעקב אחר שינויים: פעיל",
        "English (United States)" => "עברית (ישראל)", "Page" => "עמוד",
        "words" => "מילים", "Editing header/footer" => "עריכת כותרת עליונה/תחתונה",
        "Theme Colors" => "צבעי ערכת נושא", "Standard Colors" => "צבעים רגילים",
        "Not available right now" => "לא זמין כרגע",
        "Language" => "שפה", "Hebrew" => "עברית", "English" => "אנגלית",
        _ => s.to_owned(),
    }
}
