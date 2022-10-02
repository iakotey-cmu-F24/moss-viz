use clap::ValueEnum;

#[derive(Clone, Debug, Default, ValueEnum)]

pub enum OutputFormat {
    /// (Dot format containing layout infomation)
    TDOT,

    /// (Dot format containing complete layout infomation)
    XDOT,

    /// (PostScript)
    PS,

    /// (PDF)
    PDF,

    #[default]
    /// −Tsvgz (Structured Vector Graphics)
    SVG,

    /// (XFIG graphics)
    FIG,

    /// (png bitmap graphics)
    PNG,

    /// (gif bitmap graphics)
    GIF,

    /// −Tjpeg (jpeg bitmap graphics)
    JPG,

    /// (xdot information encoded in JSON)
    JSON,

    /// (imagemap files for httpd servers for each node or edge that has a non-null href attribute.)
    IMAP,

    /// (client-side imagemap for use in html and xhtml)
    CMAPX,
}
