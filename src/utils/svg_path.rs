use iced::widget::svg;

pub struct Path {
    //pub command: String,
    pub rules: Vec<String>,
    //pub stroke_width: usize,
    //pub stroke_color: (u8,u8,u8),
    pub length: usize,
}

impl Path {
    pub fn new() -> Self {
        Path {
            //command: String::new(),
            rules: Vec::new(),
            //stroke_width: 10,
            //stroke_color: (255, 0, 0),
            length: 10,
        }
    }

    /// Sets `stroke=\"YourColor\"`. If unset it will remain as `stroke="none"`.
    // pub fn set_stroke_color(&mut self, r: u8, g: u8, b: u8) {
    //     self.stroke_color = (r, g, b);
    // }

    /// Sets `stroke-width=\"YourWidth\"`. If unset it will remain as `stroke-width=\"0\"`.
    // pub fn set_stroke_width(&mut self, width: usize) {
    //     self.stroke_width = width;
    // }

    /// Adds rule `"M x y"`
    pub fn move_to(&mut self, pos: [usize; 2]) {
        self.rules.push(format!("M {} {} ", pos[0], pos[1]));
    }

    /// Adds rule `"l x y"`
    pub fn line_to(&mut self, pos: [usize; 2]) {
        self.rules.push(format!("L {} {} ", pos[0], pos[1]));
    }

    /// Replace first rule L -> M
    pub fn replace(&mut self, index: usize) {
        self.rules[index] = self.rules[0].replace("L", "M");
    }

    /// Removes the last rule
    pub fn remove_last(&mut self) {
        self.rules.pop();
    }

    /// Remove the first rule
    pub fn remove_first(&mut self) {
        self.rules.remove(0);
    }

    /// Remove all rules
    pub fn remove_all(&mut self) {
        self.rules.clear();
    }

    /// Add point to the rules
    pub fn add_point(&mut self, pos: [usize; 2]) {
        // a new line, move to
        if self.rules.len() == 0 {
            self.move_to(pos);
            return;
        }

        // larger than size, remove one, and change the new start to move
        if self.rules.len() >= self.length {
            self.remove_first();
            self.replace(0); // replace first
        }

        // add the new end point
        self.line_to(pos);
    }

    // Returns a `String` with path information. Example: `"M 0 0 L 100 100 ..."`
    pub fn create_command(&mut self) -> String {
        let mut path = String::new();
        for rule in &self.rules {
            path.push_str(rule);
        }
        path
    }
}

pub fn set_fill(svg_bytes: &[u8], color: String) -> &'static [u8] {
    let mut svg = String::from_utf8_lossy(svg_bytes).to_string();

    // Modify the fill attribute of the path
    if let Some(start) = svg.find("<path") {
        if let Some(end) = svg[start..].find("/>") {
            let end_index = start + end;
            svg.insert_str(end_index, &format!(r#" fill="{}""#, color).as_str());
        }
    }

    // leak memory by storing the svg in memory
    Box::leak(svg.into_bytes().into_boxed_slice())
}
