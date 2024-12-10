use crate::patch::Patch;
use regex::Regex;

/**
    This patch replaces TDP slider with power profile slider.
*/

fn get_js_code(react: &str, translations: &str) -> String {
    format!(r#"
        const [options, setOptions] = {0}.useState([]);
        const [value, setValue] = {0}.useState(undefined);

        {0}.useEffect(() => {{
            fetch("http://localhost:1338/settings")
                .then(response => response.json())
                .then(settings => {{
                    setOptions(settings.power_profiles.map((name, index) => ({{
                        notchIndex: index,
                        label: name,
                        value: name
                    }})))
                    setValue(settings.power_profiles.indexOf(settings.power_profile));
                }})
        }}, []);

        const onChange = (newValue) => {{
            setValue(newValue);
            fetch("http://localhost:1338/settings", {{
               headers: {{
                   'Content-Type': 'application/json'
               }},
               method: "POST",
               body: JSON.stringify({{power_profile: options[newValue]?.value}})
            }})
        }}
        
        if (value === undefined) return null;

        return {0}.createElement(i.d3, {{
          info: {{visible: true, min: 0, max: options.length - 1}},
          available: true,
          label: "Performance Mode",
          explainer: {1}("\#QuickAccess_Tab_Perf_TDPLimit_Explainer"),
          value: value,
          layout: "below",
          onChange: onChange,
          min: 0,
          max: options.length - 1,
          step: 1,
          bottomSeparator: "standard",
          notchCount: options.length,
          notchLabels: options,
          notchTicksVisible: false,
        }})
    "#, react, translations)
}

pub fn get_patch() -> Patch {
    Patch {
        regex: Regex::new(r#"(const \w{1,2}=\(\d{1,2},\w{1,2}.\w{1,2}\)\(\);return (\w{1,2})\.createElement\(\w{1,2}\.\w{1,2},\{setting:"steamos_tdp_limit".*?explainerTitle:(\(.*?\)).*?}\))"#).unwrap(),
        handler: Box::new(|captures| {
            let method = captures.get(1).map_or("", |m| m.as_str());
            let react = captures.get(2).map_or("", |m| m.as_str());
            let translations = captures.get(3).map_or("", |m| m.as_str());

            if method.is_empty() {
                return None;
            }
            
            println!("Patched TDP slider");

            Some((method.to_string(), get_js_code(&react, &translations)))
        }),
    }
}
