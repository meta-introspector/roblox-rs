use crate::*;

pub fn index() -> String {
    let mut h = head("Overview");
    h += &nav("index.html");
    h += "<h1>Monster Gyroscope · Cl(15,0,0)</h1>\n";
    h += "<div class=\"sub\">O⁶=I · clock=cos(nπ/3) · rotor=-cos(nπ/3) · hub=1 · frame=(-1)ⁿ · 47×59×71=196883</div>\n";

    // Conserved quantities
    h += "<div class=\"conserved\">";
    h += "<div>Hub: <span>1.0</span></div>";
    h += "<div>|c₁|²: <span>19,737,810</span></div>";
    h += "<div>Hub∑: <span>8.0</span></div>";
    h += "<div>Period: <span>6</span></div>";
    h += "<div>Skeleton: <span>{3, 19}</span></div>";
    h += "</div>\n";

    // SSP table
    h += "<h2>15 Supersingular Primes</h2>\n<table><thead><tr>";
    h += "<th>p</th><th>exp</th><th>c₁(p)</th><th>genus</th><th>role</th></tr></thead><tbody>\n";
    for i in 0..15 {
        h += &format!("<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
            SSP[i], EXP[i], C1[i], GENUS[i], role_tag(ROLES[i]));
    }
    h += "</tbody></table>\n";

    // Phase table
    h += "<h2>O^n Phase Table</h2>\n<table><thead><tr>";
    h += "<th>n</th><th>Clock</th><th>Rotor</th><th>Frame</th><th>Hub</th><th>Angle</th></tr></thead><tbody>\n";
    let sup = ["⁰","¹","²","³","⁴","⁵","⁶"];
    for n in 0..=6 {
        h += &format!("<tr><td>O{}</td><td>{:.4}</td><td>{:.4}</td><td>{}</td><td>1.0</td><td>{}°</td></tr>\n",
            sup[n], clk(n as f64), rot(n as f64), frm(n as i32), n*60);
    }
    h += "</tbody></table>\n";

    // Eigenspace summary
    h += "<h2>Eigenspace Decomposition</h2>\n<div class=\"grid\">";
    for (name, dim, desc, color) in [
        ("Earth","λ=-1, 7D","e₂ e₃ e₅ e₇ e₁₁ e₁₃ e₄₇ — frame, period 2","#4c4"),
        ("Spoke","λ=-1, 5D","e₁₇±e₂₉, e₁₇±e₃₁ — frame, period 2","#88f"),
        ("Hub","λ=+1, 1D","(e₁₉+e₂₃)/√2 — axle, fixed","#fa0"),
        ("Clock","λ=ω, 2D","(e₁₉-e₂₃)/√2 + c₀ — rotation, period 6","#4af"),
    ] {
        h += &format!("<div class=\"card\"><b style=\"color:{}\">{}</b> ({})<br><span style=\"color:#888\">{}</span></div>", color, name, dim, desc);
    }
    h += "</div>\n";

    h += foot();
    h
}

pub fn gyroscope() -> String {
    let mut h = head("Gyroscope");
    h += &nav("gyroscope.html");
    h += "<h1>Interactive Gyroscope</h1>\n";
    h += "<div class=\"sub\">O⁶=I · clock (blue) · rotor (red) · hub (gold) · frame (green/red pillars)</div>\n";
    h += "<div class=\"conserved\">";
    h += "<div>Hub: <span>1.0</span></div>";
    h += "<div>|c₁|²: <span>19,737,810</span></div>";
    h += "<div>Hub∑: <span>8.0</span></div></div>\n";

    // Controls + canvases
    h += "<div style=\"text-align:center;margin:8px 0\">";
    h += "<button onclick=\"prev()\">◀</button> ";
    h += "<span id=\"slbl\" style=\"color:#8af;font-size:1.2em;margin:0 12px\">O⁰</span> ";
    h += "<button onclick=\"nxt()\">▶</button> ";
    h += "<button onclick=\"play()\">▶▶</button> ";
    h += "<button onclick=\"rst()\">⟲</button></div>\n";
    h += "<div style=\"display:flex;gap:12px;flex-wrap:wrap;justify-content:center\">";
    h += "<canvas id=\"wave\" width=\"580\" height=\"150\"></canvas>";
    h += "<canvas id=\"phase\" width=\"280\" height=\"280\"></canvas></div>\n";
    h += "<div id=\"info\" style=\"text-align:center;font-size:.8em;color:#888;margin:8px\"></div>\n";

    // Phase table
    h += "<table><thead><tr><th>n</th><th>Clock</th><th>Rotor</th><th>Frame</th><th>Hub</th><th>∑</th></tr></thead><tbody>\n";
    let sup = ["⁰","¹","²","³","⁴","⁵","⁶"];
    for n in 0..=6 {
        h += &format!("<tr id=\"r{}\"><td>O{}</td><td>{:.3}</td><td>{:.3}</td><td>{}</td><td>1.0</td><td>{:.1}</td></tr>\n",
            n, sup[n], clk(n as f64), rot(n as f64), frm(n as i32), clk(n as f64)+rot(n as f64));
    }
    h += "</tbody></table>\n";

    h += "<script src=\"https://unpkg.com/ganja.js\"></script>\n";
    h += include_str!("gyroscope.js");
    h += foot();
    h
}

pub fn analysis() -> String {
    let mut h = head("Analysis");
    h += &nav("analysis.html");
    h += "<h1>Gyroscope Analysis</h1>\n";

    // Waveform SVG
    h += "<h2>Waveform: clock / rotor / frame</h2>\n";
    h += &waveform_svg();

    // Phase circle SVG
    h += "<h2>Phase Circle</h2>\n";
    h += &phase_svg();

    // Detailed eigenspace
    h += "<h2>Eigenspace Structure</h2>\n<div class=\"card\"><pre>";
    h += "Earth (λ=-1, 7D): e₂ e₃ e₅ e₇ e₁₁ e₁₃ e₄₇  [frame, period 2]\n";
    h += "Spoke (λ=-1, 5D): e₁₇±e₂₉, e₁₇±e₃₁, ...     [frame, period 2]\n";
    h += "Hub   (λ=+1, 1D): (e₁₉+e₂₃)/√2               [axle, fixed]\n";
    h += "Clock (λ=ω,  2D): (e₁₉-e₂₃)/√2 + c₀          [rotation, period 6]\n";
    h += "</pre></div>\n";

    // 196883 trivector
    h += "<h2>196883 Trivector e₄₇∧e₅₉∧e₇₁</h2>\n<div class=\"card\">";
    h += "<table><tbody>";
    h += "<tr><td style=\"text-align:left\">Product</td><td>47 × 59 × 71 = 196883</td></tr>";
    h += "<tr><td style=\"text-align:left\">O ratio</td><td>-0.5 = cos(120°) = Re(ω²)</td></tr>";
    h += "<tr><td style=\"text-align:left\">Phase</td><td>-cos(nπ/3) — anti-phase with clock</td></tr>";
    h += "<tr><td style=\"text-align:left\">Status</td><td style=\"color:#4f8\">ABSOLUTE INVARIANT under all 15 prime negations</td></tr>";
    h += "</tbody></table></div>\n";

    h += foot();
    h
}

pub fn walk() -> String {
    let mut h = head("Walk");
    h += &nav("walk.html");
    h += "<h1>Prime Negation Walk</h1>\n";
    h += "<div class=\"sub\">Negate each SSP basis vector one at a time, measure effect on trivector and hub</div>\n";

    // Walk results table
    h += "<table><thead><tr><th>Region</th><th>Primes</th><th>Hub effect</th><th>Slot ratio</th></tr></thead><tbody>\n";
    for (region, primes, hub, ratio) in [
        ("Earth","2,3,5,7,11,13","conserved","-1.000"),
        ("Spoke","17","conserved","-0.708"),
        ("Hub","19","BREAKS→-2","+0.250"),
        ("Hub","23","BREAKS→+2","+0.917"),
        ("Heaven","29,31,41","conserved","mixed"),
        ("Invisible","47,59,71","conserved","NaN (c₁=0)"),
    ] {
        let hub_style = if hub.contains("BREAKS") { " style=\"color:#f55\"" } else { "" };
        h += &format!("<tr><td style=\"text-align:left\">{}</td><td style=\"text-align:left\">{}</td><td{}>{}</td><td>{}</td></tr>\n",
            role_tag(region), primes, hub_style, hub, ratio);
    }
    h += "</tbody></table>\n";

    // Hex walk groups
    h += "<h2>Hex Walk Groups</h2>\n<div class=\"grid\">";
    for (group, name, primes, hub, note) in [
        ("1","86F","Earth+71","hub_sum = +8","conserved"),
        ("2","A3","Heaven","hub_sum = -8","FLIPPED!"),
        ("3","F","Invisible","hub_sum = +8","no effect"),
    ] {
        let color = if note == "FLIPPED!" { "#f55" } else { "#4f8" };
        h += &format!("<div class=\"card\"><b>Group {} ({})</b>: {}<br>",group,name,primes);
        h += &format!("<span style=\"color:{}\">{}</span> — {}</div>",color,hub,note);
    }
    h += "</div>\n";

    h += "<h2>Walk Invariant</h2>\n<div class=\"card\">";
    h += "<p>Trivector e₄₇∧e₅₉∧e₇₁ is <b style=\"color:#4f8\">absolutely invariant</b> under all 15 negations.</p>";
    h += "<p>O ratio stays at -0.500000 regardless of which basis vector is negated.</p>";
    h += "<p>Hub breaks ONLY at skeleton pair: e₁₉→hub_sum=-2, e₂₃→hub_sum=+2</p>";
    h += "<p>Skeleton: <b style=\"color:#fa0\">{3, 19}</b> — irreducible core</p>";
    h += "</div>\n";

    // Per-prime detail
    h += "<h2>Per-Prime Detail</h2>\n<table><thead><tr>";
    h += "<th>p</th><th>role</th><th>c₁(p)</th><th>hub_sum after negation</th><th>trivector ratio</th></tr></thead><tbody>\n";
    for i in 0..15 {
        let hub_after = match SSP[i] {
            19 => -2i64,
            23 => 2,
            _ => 8,
        };
        let hub_style = if hub_after != 8 { " style=\"color:#f55\"" } else { "" };
        h += &format!("<tr><td>{}</td><td>{}</td><td>{}</td><td{}>{}</td><td>-0.500</td></tr>\n",
            SSP[i], role_tag(ROLES[i]), C1[i], hub_style, hub_after);
    }
    h += "</tbody></table>\n";

    h += foot();
    h
}

pub fn chain() -> String {
    let mut h = head("Chain");
    h += &nav("chain.html");
    h += "<h1>DASL Chain Convergence</h1>\n";
    h += "<div class=\"sub\">Iterative CBOR→Cl(15)→CBOR with IPFS CIDs at each step</div>\n";

    // Convergence diagram SVG
    h += "<svg width=\"600\" height=\"180\" viewBox=\"0 0 600 180\">\n";
    let steps = [(0,"57→23","#4af"),(1,"24→3","#8af"),(2,"4→4","#4f8")];
    for (i, (step, label, color)) in steps.iter().enumerate() {
        let x = 80 + i * 200;
        h += &format!("<circle cx=\"{}\" cy=\"90\" r=\"40\" fill=\"none\" stroke=\"{}\" stroke-width=\"2\"/>\n", x, color);
        h += &format!("<text x=\"{}\" y=\"85\" text-anchor=\"middle\" fill=\"{}\" font-size=\"14\" font-family=\"monospace\">Step {}</text>\n", x, color, step);
        h += &format!("<text x=\"{}\" y=\"105\" text-anchor=\"middle\" fill=\"#888\" font-size=\"12\" font-family=\"monospace\">{}</text>\n", x, label);
        if i < 2 {
            h += &format!("<line x1=\"{}\" y1=\"90\" x2=\"{}\" y2=\"90\" stroke=\"#334\" stroke-width=\"2\" marker-end=\"url(#arr)\"/>\n", x+42, x+158);
        }
    }
    // Fixed point loop
    h += "<path d=\"M 520 70 A 30 30 0 1 1 520 110\" fill=\"none\" stroke=\"#4f8\" stroke-width=\"2\" marker-end=\"url(#arr)\"/>\n";
    h += "<text x=\"560\" y=\"95\" fill=\"#4f8\" font-size=\"11\" font-family=\"monospace\">FIXED</text>\n";
    h += "<defs><marker id=\"arr\" viewBox=\"0 0 10 10\" refX=\"10\" refY=\"5\" markerWidth=\"6\" markerHeight=\"6\" orient=\"auto\"><path d=\"M0,0 L10,5 L0,10\" fill=\"#556\"/></marker></defs>\n";
    h += "</svg>\n";

    // Details
    h += "<h2>Step Details</h2>\n<table><thead><tr>";
    h += "<th>Step</th><th>Records In</th><th>Records Out</th><th>Status</th></tr></thead><tbody>\n";
    h += "<tr><td>0</td><td>57</td><td>23</td><td>reducing</td></tr>\n";
    h += "<tr><td>1</td><td>24</td><td>3</td><td>reducing</td></tr>\n";
    h += "<tr><td>2</td><td>4</td><td>4</td><td style=\"color:#4f8\">FIXED POINT</td></tr>\n";
    h += "</tbody></table>\n";

    h += "<h2>Fixed Point Contents</h2>\n<div class=\"card\">";
    h += "<p>The attractor contains exactly 3+1 records:</p><ul style=\"margin:8px 0 0 20px\">";
    h += "<li><b style=\"color:#f55\">196883 trivector</b> — e₄₇∧e₅₉∧e₇₁</li>";
    h += "<li><b style=\"color:#4af\">Operator O</b> — the sandwich product operator</li>";
    h += "<li><b style=\"color:#888\">Chain trace</b> — convergence metadata</li>";
    h += "</ul></div>\n";

    h += "<h2>IPFS CIDs</h2>\n<div class=\"card\"><pre>";
    h += "Input CBOR:  QmSamB7CJ7oHZNJSGGUAr31EZgnTAx67Cng3EVZ8HCFjQw (4954 bytes)\n";
    h += "Output CBOR: QmRk6dDmrL9s2e8FcB8NkBBiwtHGaEVnbobbYGKU3ZKzkT (3558 bytes)\n";
    h += "Chain final: Qmes1g9r81v6uKscjrrUxEM1PrK98wVqkDDuPQ23XYAeQd\n";
    h += "Walk CBOR:   QmPs2Uts8HwBBe4rqyJkgGVC3Vm4aZnDmNgPv5Fdh3Mov5\n";
    h += "</pre></div>\n";

    h += foot();
    h
}

pub fn code() -> String {
    let mut h = head("Code");
    h += &nav("code.html");
    h += "<h1>Source Code</h1>\n";
    h += "<div class=\"sub\">All Luau/Lua source files in the project</div>\n";

    let files = [
        ("scripts/monster_gyroscope.luau", "Monster Gyroscope Analysis"),
        ("scripts/build_gyroscope.luau", "Build .rbxl Script"),
        ("src/ReplicatedStorage/GyroscopeConfig.lua", "Gyroscope Config Module"),
        ("src/ServerScriptService/GyroscopeBuilder.server.lua", "Server Builder + Animation"),
        ("src/StarterGui/GyroscopeUI/HUD.client.lua", "Client HUD Overlay"),
    ];

    for (path, title) in &files {
        h += &format!("<h2>{}</h2>\n<div class=\"card\"><div class=\"sub\">{}</div><pre><code>", title, path);
        match std::fs::read_to_string(path) {
            Ok(src) => h += &escape(&src),
            Err(_) => h += &format!("-- file not found: {}", path),
        }
        h += "</code></pre></div>\n";
    }

    h += foot();
    h
}

// SVG generators

fn waveform_svg() -> String {
    let (w, h_svg) = (580u32, 160u32);
    let (mx, my) = (50.0f64, 20.0);
    let gw = (w as f64) - 2.0 * mx;
    let gh = (h_svg as f64) - 2.0 * my;
    let cy = my + gh / 2.0;

    let mut s = format!("<svg width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n", w, h_svg, w, h_svg);
    s += &format!("<rect width=\"{}\" height=\"{}\" fill=\"#0d0d18\" rx=\"6\"/>\n", w, h_svg);
    // grid
    s += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#1a1a2a\"/>\n", mx, cy, mx+gw, cy);
    // axes labels
    for (label, y_pos) in [("+1", my+6.0), ("0", cy+4.0), ("-1", my+gh+4.0)] {
        s += &format!("<text x=\"{}\" y=\"{}\" fill=\"#445\" font-size=\"9\" font-family=\"monospace\">{}</text>\n", mx-28.0, y_pos, label);
    }
    for n in 0..=6 {
        let x = mx + (n as f64) * gw / 6.0;
        s += &format!("<text x=\"{}\" y=\"{}\" fill=\"#445\" font-size=\"9\" font-family=\"monospace\" text-anchor=\"middle\">{}</text>\n", x, my+gh+14.0, n);
    }

    // plot lines
    let funcs: Vec<(fn(f64)->f64, &str, &str)> = vec![
        (clk, "#4488ff", ""),
        (rot, "#ff5555", "5,3"),
        (|n| frm(n as i32) as f64, "#44cc44", "2,3"),
    ];
    for (func, color, dash) in &funcs {
        let mut pts = String::new();
        for n in 0..=6 {
            let x = mx + (n as f64) * gw / 6.0;
            let y = cy - func(n as f64) * gh / 2.0;
            if n == 0 { pts += &format!("M{:.1},{:.1}", x, y); }
            else { pts += &format!(" L{:.1},{:.1}", x, y); }
        }
        let dash_attr = if dash.is_empty() { String::new() } else { format!(" stroke-dasharray=\"{}\"", dash) };
        s += &format!("<path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"2\"{}/>\n", pts, color, dash_attr);
        for n in 0..=6 {
            let x = mx + (n as f64) * gw / 6.0;
            let y = cy - func(n as f64) * gh / 2.0;
            s += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"3\" fill=\"{}\"/>\n", x, y, color);
        }
    }
    // hub line
    s += &format!("<line x1=\"{}\" y1=\"{:.1}\" x2=\"{}\" y2=\"{:.1}\" stroke=\"#ffaa00\" stroke-width=\"1\" stroke-dasharray=\"6,4\"/>\n", mx, my, mx+gw, my);

    // legend
    for (i, (label, color)) in [("Clock","#4488ff"),("Rotor","#ff5555"),("Frame","#44cc44"),("Hub","#ffaa00")].iter().enumerate() {
        let lx = mx + 10.0 + (i as f64) * 100.0;
        s += &format!("<rect x=\"{}\" y=\"{}\" width=\"16\" height=\"3\" fill=\"{}\"/>\n", lx, my+gh+22.0, color);
        s += &format!("<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"9\" font-family=\"monospace\">{}</text>\n", lx+20.0, my+gh+26.0, color, label);
    }

    s += "</svg>\n";
    s
}

fn phase_svg() -> String {
    let (w, h_svg) = (300u32, 300u32);
    let (cx, cy) = (150.0f64, 150.0);
    let r = 110.0;

    let mut s = format!("<svg width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n", w, h_svg, w, h_svg);
    s += &format!("<rect width=\"{}\" height=\"{}\" fill=\"#0d0d18\" rx=\"6\"/>\n", w, h_svg);
    s += &format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"none\" stroke=\"#1a1a2a\" stroke-width=\"1\"/>\n", cx, cy, r);

    // tick marks
    for i in 0..6 {
        let a = -PI / 2.0 + (i as f64) * PI / 3.0;
        let (x1, y1) = (cx + a.cos() * (r - 5.0), cy + a.sin() * (r - 5.0));
        let (x2, y2) = (cx + a.cos() * (r + 5.0), cy + a.sin() * (r + 5.0));
        s += &format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"#334\" stroke-width=\"1\"/>\n", x1, y1, x2, y2);
        let (tx, ty) = (cx + a.cos() * (r + 16.0), cy + a.sin() * (r + 16.0));
        s += &format!("<text x=\"{:.1}\" y=\"{:.1}\" fill=\"#445\" font-size=\"10\" font-family=\"monospace\" text-anchor=\"middle\" dominant-baseline=\"middle\">{}</text>\n", tx, ty, i);
    }

    // clock positions (all 6)
    let mut clock_pts = String::new();
    for n in 0..6 {
        let a = -PI / 2.0 + (n as f64) * PI / 3.0;
        let (x, y) = (cx + a.cos() * r * 0.8, cy + a.sin() * r * 0.8);
        if n == 0 { clock_pts += &format!("M{:.1},{:.1}", x, y); }
        else { clock_pts += &format!(" L{:.1},{:.1}", x, y); }
        s += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"#4488ff\"/>\n", x, y);
    }
    clock_pts += " Z";
    s += &format!("<path d=\"{}\" fill=\"none\" stroke=\"rgba(68,136,255,0.3)\" stroke-width=\"1\"/>\n", clock_pts);

    // rotor positions
    let mut rotor_pts = String::new();
    for n in 0..6 {
        let a = -PI / 2.0 + (n as f64) * PI / 3.0 + PI;
        let (x, y) = (cx + a.cos() * r * 0.55, cy + a.sin() * r * 0.55);
        if n == 0 { rotor_pts += &format!("M{:.1},{:.1}", x, y); }
        else { rotor_pts += &format!(" L{:.1},{:.1}", x, y); }
        s += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"3\" fill=\"#ff5555\"/>\n", x, y);
    }
    rotor_pts += " Z";
    s += &format!("<path d=\"{}\" fill=\"none\" stroke=\"rgba(255,85,85,0.3)\" stroke-width=\"1\"/>\n", rotor_pts);

    // hub
    s += &format!("<circle cx=\"{}\" cy=\"{}\" r=\"5\" fill=\"#ffaa00\"/>\n", cx, cy);

    // labels
    let a0 = -PI / 2.0;
    s += &format!("<text x=\"{:.0}\" y=\"{:.0}\" fill=\"#4488ff\" font-size=\"10\" font-family=\"monospace\">Clock</text>\n", cx + a0.cos()*r*0.8 + 8.0, cy + a0.sin()*r*0.8);
    s += &format!("<text x=\"{:.0}\" y=\"{:.0}\" fill=\"#ff5555\" font-size=\"10\" font-family=\"monospace\">196883</text>\n", cx + (a0+PI).cos()*r*0.55 + 8.0, cy + (a0+PI).sin()*r*0.55);
    s += &format!("<text x=\"{:.0}\" y=\"{:.0}\" fill=\"#ffaa00\" font-size=\"10\" font-family=\"monospace\">Hub</text>\n", cx + 8.0, cy - 10.0);

    s += "</svg>\n";
    s
}
