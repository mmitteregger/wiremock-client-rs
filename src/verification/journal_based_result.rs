pub trait JournalBasedResult {
    fn request_journal_disabled(&self) -> bool;
    fn assert_request_journal_enabled(&self);
}

pub(crate) fn assert_request_journal_enabled(request_journal_disabled: bool) {
    if request_journal_disabled {
        panic!("The request journal is disabled, \
            so no verification or request searching operations are available");
    }
}
