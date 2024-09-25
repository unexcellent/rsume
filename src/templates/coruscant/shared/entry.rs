pub struct Entry {
    pub start_date: String,
    pub end_date: String,
    pub title: String,
    pub body: String,
}
impl Entry {
    pub fn build(&self) -> String {
        let end_date = &self.end_date;
        let start_date = &self.start_date;
        let title = &self.title;
        let body = &self.body;

        let html = format!(
            "
            <div class='entry'>
                <div class='entry-inner'>
                    <div class='timespan-column'>
                        <div class='end-date'>
                            {end_date}
                        </div>
                        <div class='start-date'>
                            {start_date}
                        </div>
                    </div>
                    <div class='timeline'>
                        <div class='top-circle'></div>
                        <div class='line'></div>
                        <div class='bottom-circle'></div>
                    </div>
                    <div class='box-column'>
                            <div class='entry-title'>
                                {title}
                            </div>
                            <div class='entry-body'>
                                {body}
                            </div>
                    </div>
                </div>
            </div>
        "
        );

        html
    }
}
