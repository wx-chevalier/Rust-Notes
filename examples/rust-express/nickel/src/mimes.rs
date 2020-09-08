use hyper::mime::Mime;
use std::str::FromStr;

macro_rules! mimes {
    ($($t:expr => { $($name:ident, $as_s:pat, $subt:expr,)+ })+) => (
        #[allow(non_camel_case_types)]
        #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
        pub enum MediaType {
            $(
                $(
                    $name
                ),*
            ),*
        }

        // FIXME: Should be less runtime cost to this, hyper's Mime type looks
        // slightly more robust than old-http, so could probably just re-export
        // that and depreciate this.
        impl From<MediaType> for Mime {
            fn from(mt: MediaType) -> Mime {
                match mt {
                    $(
                        $(
                            MediaType::$name => concat!($t, "/", $subt)
                        ),*
                    ),*
                }.parse().unwrap()
            }
        }

        impl FromStr for MediaType {
            type Err = &'static str;
            fn from_str(s: &str) -> Result<MediaType, &'static str> {
                Ok(match s {
                    $(
                        $(
                            $as_s => MediaType::$name
                        ),*
                    ),*,
                    _ => return Err("Not a valid MediaType.")
                })
            }
        }
    )
}

mimes!(

    "application" => {

        Ez, "ez", "andrew-inset",
        Aw, "aw", "applixware",
        Atom, "atom", "atom+xml",
        Atomcat, "atomcat", "atomcat+xml",
        Atomsvc, "atomsvc", "atomsvc+xml",
        Ccxml, "ccxml", "ccxml+xml",
        Cdmia, "cdmia", "cdmi-capability",
        Cdmic, "cdmic", "cdmi-container",
        Cdmid, "cdmid", "cdmi-domain",
        Cdmio, "cdmio", "cdmi-object",
        Cdmiq, "cdmiq", "cdmi-queue",
        Cu, "cu", "cu-seeme",
        Davmount, "davmount", "davmount+xml",
        Dbk, "dbk", "docbook+xml",
        Dssc, "dssc", "dssc+der",
        Xdssc, "xdssc", "dssc+xml",
        Ecma, "ecma", "ecmascript",
        Emma, "emma", "emma+xml",
        Epub, "epub", "epub+zip",
        Exi, "exi", "exi",
        Pfr, "pfr", "font-tdpfr",
        Gml, "gml", "gml+xml",
        Gpx, "gpx", "gpx+xml",
        Gxf, "gxf", "gxf",
        Stk, "stk", "hyperstudio",
        Ink, "ink", "inkml+xml",
        Ipfix, "ipfix", "ipfix",
        Jar, "jar", "java-archive",
        Ser, "ser", "java-serialized-object",
        Class, "class", "java-vm",
        Js, "js", "javascript",
        Json, "json", "json",
        Jsonml, "jsonml", "jsonml+json",
        Lostxml, "lostxml", "lost+xml",
        Hqx, "hqx", "mac-binhex40",
        Cpt, "cpt", "mac-compactpro",
        Mads, "mads", "mads+xml",
        Mrc, "mrc", "marc",
        Mrcx, "mrcx", "marcxml+xml",
        Ma, "ma", "mathematica",
        Mathml, "mathml", "mathml+xml",
        Mbox, "mbox", "mbox",
        Mscml, "mscml", "mediaservercontrol+xml",
        Metalink, "metalink", "metalink+xml",
        Meta4, "meta4", "metalink4+xml",
        Mets, "mets", "mets+xml",
        Mods, "mods", "mods+xml",
        M21, "m21", "mp21",
        Mp4s, "mp4s", "mp4",
        Doc, "doc", "msword",
        Mxf, "mxf", "mxf",
        Bin, "bin", "octet-stream",
        Oda, "oda", "oda",
        Opf, "opf", "oebps-package+xml",
        Ogx, "ogx", "ogg",
        Omdoc, "omdoc", "omdoc+xml",
        Onetoc, "onetoc", "onenote",
        Oxps, "oxps", "oxps",
        Xer, "xer", "patch-ops-error+xml",
        Pdf, "pdf", "pdf",
        Pgp, "pgp", "pgp-encrypted",
        Asc, "asc", "pgp-signature",
        Prf, "prf", "pics-rules",
        P10, "p10", "pkcs10",
        P7m, "p7m", "pkcs7-mime",
        P7s, "p7s", "pkcs7-signature",
        P8, "p8", "pkcs8",
        Ac, "ac", "pkix-attr-cert",
        Cer, "cer", "pkix-cert",
        Crl, "crl", "pkix-crl",
        Pkipath, "pkipath", "pkix-pkipath",
        Pki, "pki", "pkixcmp",
        Pls, "pls", "pls+xml",
        Ai, "ai", "postscript",
        Cww, "cww", "prs.cww",
        Pskcxml, "pskcxml", "pskc+xml",
        Rdf, "rdf", "rdf+xml",
        Rif, "rif", "reginfo+xml",
        Rnc, "rnc", "relax-ng-compact-syntax",
        Rl, "rl", "resource-lists+xml",
        Rld, "rld", "resource-lists-diff+xml",
        Rs, "rs", "rls-services+xml",
        Gbr, "gbr", "rpki-ghostbusters",
        Mft, "mft", "rpki-manifest",
        Roa, "roa", "rpki-roa",
        Rsd, "rsd", "rsd+xml",
        Rss, "rss", "rss+xml",
        Rtf, "rtf", "rtf",
        Sbml, "sbml", "sbml+xml",
        Scq, "scq", "scvp-cv-request",
        Scs, "scs", "scvp-cv-response",
        Spq, "spq", "scvp-vp-request",
        Spp, "spp", "scvp-vp-response",
        Sdp, "sdp", "sdp",
        Setpay, "setpay", "set-payment-initiation",
        Setreg, "setreg", "set-registration-initiation",
        Shf, "shf", "shf+xml",
        Smi, "smi", "smil+xml",
        Rq, "rq", "sparql-query",
        Srx, "srx", "sparql-results+xml",
        Gram, "gram", "srgs",
        Grxml, "grxml", "srgs+xml",
        Sru, "sru", "sru+xml",
        Ssdl, "ssdl", "ssdl+xml",
        Ssml, "ssml", "ssml+xml",
        Tei, "tei", "tei+xml",
        Tfi, "tfi", "thraud+xml",
        Tsd, "tsd", "timestamped-data",
        Plb, "plb", "vnd.3gpp.pic-bw-large",
        Psb, "psb", "vnd.3gpp.pic-bw-small",
        Pvb, "pvb", "vnd.3gpp.pic-bw-var",
        Tcap, "tcap", "vnd.3gpp2.tcap",
        Pwn, "pwn", "vnd.3m.post-it-notes",
        Aso, "aso", "vnd.accpac.simply.aso",
        Imp, "imp", "vnd.accpac.simply.imp",
        Acu, "acu", "vnd.acucobol",
        Atc, "atc", "vnd.acucorp",
        Air, "air", "vnd.adobe.air-application-installer-package+zip",
        Fcdt, "fcdt", "vnd.adobe.formscentral.fcdt",
        Fxp, "fxp", "vnd.adobe.fxp",
        Xdp, "xdp", "vnd.adobe.xdp+xml",
        Xfdf, "xfdf", "vnd.adobe.xfdf",
        Ahead, "ahead", "vnd.ahead.space",
        Azf, "azf", "vnd.airzip.filesecure.azf",
        Azs, "azs", "vnd.airzip.filesecure.azs",
        Azw, "azw", "vnd.amazon.ebook",
        Acc, "acc", "vnd.americandynamics.acc",
        Ami, "ami", "vnd.amiga.ami",
        Apk, "apk", "vnd.android.package-archive",
        Cii, "cii", "vnd.anser-web-certificate-issue-initiation",
        Fti, "fti", "vnd.anser-web-funds-transfer-initiation",
        Atx, "atx", "vnd.antix.game-component",
        Mpkg, "mpkg", "vnd.apple.installer+xml",
        M3u8, "m3u8", "vnd.apple.mpegurl",
        Swi, "swi", "vnd.aristanetworks.swi",
        Iota, "iota", "vnd.astraea-software.iota",
        Aep, "aep", "vnd.audiograph",
        Mpm, "mpm", "vnd.blueice.multipass",
        Bmi, "bmi", "vnd.bmi",
        Rep, "rep", "vnd.businessobjects",
        Cdxml, "cdxml", "vnd.chemdraw+xml",
        Mmd, "mmd", "vnd.chipnuts.karaoke-mmd",
        Cdy, "cdy", "vnd.cinderella",
        Cla, "cla", "vnd.claymore",
        Rp9, "rp9", "vnd.cloanto.rp9",
        C4g, "c4g", "vnd.clonk.c4group",
        C11amc, "c11amc", "vnd.cluetrust.cartomobile-config",
        C11amz, "c11amz", "vnd.cluetrust.cartomobile-config-pkg",
        Csp, "csp", "vnd.commonspace",
        Cdbcmsg, "cdbcmsg", "vnd.contact.cmsg",
        Cmc, "cmc", "vnd.cosmocaller",
        Clkx, "clkx", "vnd.crick.clicker",
        Clkk, "clkk", "vnd.crick.clicker.keyboard",
        Clkp, "clkp", "vnd.crick.clicker.palette",
        Clkt, "clkt", "vnd.crick.clicker.template",
        Clkw, "clkw", "vnd.crick.clicker.wordbank",
        Wbs, "wbs", "vnd.criticaltools.wbs+xml",
        Pml, "pml", "vnd.ctc-posml",
        Ppd, "ppd", "vnd.cups-ppd",
        Car, "car", "vnd.curl.car",
        Pcurl, "pcurl", "vnd.curl.pcurl",
        Dart, "dart", "vnd.dart",
        Rdz, "rdz", "vnd.data-vision.rdz",
        Uvf, "uvf", "vnd.dece.data",
        Uvt, "uvt", "vnd.dece.ttml+xml",
        Uvx, "uvx", "vnd.dece.unspecified",
        Uvz, "uvz", "vnd.dece.zip",
        Fe_launch, "fe_launch", "vnd.denovo.fcselayout-link",
        Dna, "dna", "vnd.dna",
        Mlp, "mlp", "vnd.dolby.mlp",
        Dpg, "dpg", "vnd.dpgraph",
        Dfac, "dfac", "vnd.dreamfactory",
        Kpxx, "kpxx", "vnd.ds-keypoint",
        Ait, "ait", "vnd.dvb.ait",
        Svc, "svc", "vnd.dvb.service",
        Geo, "geo", "vnd.dynageo",
        Mag, "mag", "vnd.ecowin.chart",
        Nml, "nml", "vnd.enliven",
        Esf, "esf", "vnd.epson.esf",
        Msf, "msf", "vnd.epson.msf",
        Qam, "qam", "vnd.epson.quickanime",
        Slt, "slt", "vnd.epson.salt",
        Ssf, "ssf", "vnd.epson.ssf",
        Es3, "es3", "vnd.eszigno3+xml",
        Ez2, "ez2", "vnd.ezpix-album",
        Ez3, "ez3", "vnd.ezpix-package",
        Fdf, "fdf", "vnd.fdf",
        Mseed, "mseed", "vnd.fdsn.mseed",
        Seed, "seed", "vnd.fdsn.seed",
        Gph, "gph", "vnd.flographit",
        Ftc, "ftc", "vnd.fluxtime.clip",
        Fm, "fm", "vnd.framemaker",
        Fnc, "fnc", "vnd.frogans.fnc",
        Ltf, "ltf", "vnd.frogans.ltf",
        Fsc, "fsc", "vnd.fsc.weblaunch",
        Oas, "oas", "vnd.fujitsu.oasys",
        Oa2, "oa2", "vnd.fujitsu.oasys2",
        Oa3, "oa3", "vnd.fujitsu.oasys3",
        Fg5, "fg5", "vnd.fujitsu.oasysgp",
        Bh2, "bh2", "vnd.fujitsu.oasysprs",
        Ddd, "ddd", "vnd.fujixerox.ddd",
        Xdw, "xdw", "vnd.fujixerox.docuworks",
        Xbd, "xbd", "vnd.fujixerox.docuworks.binder",
        Fzs, "fzs", "vnd.fuzzysheet",
        Txd, "txd", "vnd.genomatix.tuxedo",
        Ggb, "ggb", "vnd.geogebra.file",
        Ggt, "ggt", "vnd.geogebra.tool",
        Gex, "gex", "vnd.geometry-explorer",
        Gxt, "gxt", "vnd.geonext",
        G2w, "g2w", "vnd.geoplan",
        G3w, "g3w", "vnd.geospace",
        Gmx, "gmx", "vnd.gmx",
        Kml, "kml", "vnd.google-earth.kml+xml",
        Kmz, "kmz", "vnd.google-earth.kmz",
        Gqf, "gqf", "vnd.grafeq",
        Gac, "gac", "vnd.groove-account",
        Ghf, "ghf", "vnd.groove-help",
        Gim, "gim", "vnd.groove-identity-message",
        Grv, "grv", "vnd.groove-injector",
        Gtm, "gtm", "vnd.groove-tool-message",
        Tpl, "tpl", "vnd.groove-tool-template",
        Vcg, "vcg", "vnd.groove-vcard",
        Hal, "hal", "hal+json",
        HalXml, "halxml", "hal+xml",
        HalVnd, "halvnd", "vnd.hal+xml",
        Zmm, "zmm", "vnd.handheld-entertainment+xml",
        Hbci, "hbci", "vnd.hbci",
        Les, "les", "vnd.hhe.lesson-player",
        Hpgl, "hpgl", "vnd.hp-hpgl",
        Hpid, "hpid", "vnd.hp-hpid",
        Hps, "hps", "vnd.hp-hps",
        Jlt, "jlt", "vnd.hp-jlyt",
        Pcl, "pcl", "vnd.hp-pcl",
        Pclxl, "pclxl", "vnd.hp-pclxl",
        Sfd_hdstx, "sfd-hdstx", "vnd.hydrostatix.sof-data",
        Mpy, "mpy", "vnd.ibm.minipay",
        Afp, "afp", "vnd.ibm.modcap",
        Irm, "irm", "vnd.ibm.rights-management",
        Sc, "sc", "vnd.ibm.secure-container",
        Icc, "icc", "vnd.iccprofile",
        Igl, "igl", "vnd.igloader",
        Ivp, "ivp", "vnd.immervision-ivp",
        Ivu, "ivu", "vnd.immervision-ivu",
        Igm, "igm", "vnd.insors.igm",
        Xpw, "xpw", "vnd.intercon.formnet",
        I2g, "i2g", "vnd.intergeo",
        Qbo, "qbo", "vnd.intu.qbo",
        Qfx, "qfx", "vnd.intu.qfx",
        Rcprofile, "rcprofile", "vnd.ipunplugged.rcprofile",
        Irp, "irp", "vnd.irepository.package+xml",
        Xpr, "xpr", "vnd.is-xpr",
        Fcs, "fcs", "vnd.isac.fcs",
        Jam, "jam", "vnd.jam",
        Rms, "rms", "vnd.jcp.javame.midlet-rms",
        Jisp, "jisp", "vnd.jisp",
        Joda, "joda", "vnd.joost.joda-archive",
        Ktz, "ktz", "vnd.kahootz",
        Karbon, "karbon", "vnd.kde.karbon",
        Chrt, "chrt", "vnd.kde.kchart",
        Kfo, "kfo", "vnd.kde.kformula",
        Flw, "flw", "vnd.kde.kivio",
        Kon, "kon", "vnd.kde.kontour",
        Kpr, "kpr", "vnd.kde.kpresenter",
        Ksp, "ksp", "vnd.kde.kspread",
        Kwd, "kwd", "vnd.kde.kword",
        Htke, "htke", "vnd.kenameaapp",
        Kia, "kia", "vnd.kidspiration",
        Kne, "kne", "vnd.kinar",
        Skp, "skp", "vnd.koan",
        Sse, "sse", "vnd.kodak-descriptor",
        Lasxml, "lasxml", "vnd.las.las+xml",
        Lbd, "lbd", "vnd.llamagraphics.life-balance.desktop",
        Lbe, "lbe", "vnd.llamagraphics.life-balance.exchange+xml",
        Media123, "123", "vnd.lotus-1-2-3",
        Apr, "apr", "vnd.lotus-approach",
        Pre, "pre", "vnd.lotus-freelance",
        Nsf, "nsf", "vnd.lotus-notes",
        Org, "org", "vnd.lotus-organizer",
        Scm, "scm", "vnd.lotus-screencam",
        Lwp, "lwp", "vnd.lotus-wordpro",
        Portpkg, "portpkg", "vnd.macports.portpkg",
        Mcd, "mcd", "vnd.mcd",
        Mc1, "mc1", "vnd.medcalcdata",
        Cdkey, "cdkey", "vnd.mediastation.cdkey",
        Mwf, "mwf", "vnd.mfer",
        Mfm, "mfm", "vnd.mfmp",
        Flo, "flo", "vnd.micrografx.flo",
        Igx, "igx", "vnd.micrografx.igx",
        Mif, "mif", "vnd.mif",
        Daf, "daf", "vnd.mobius.daf",
        Dis, "dis", "vnd.mobius.dis",
        Mbk, "mbk", "vnd.mobius.mbk",
        Mqy, "mqy", "vnd.mobius.mqy",
        Msl, "msl", "vnd.mobius.msl",
        Plc, "plc", "vnd.mobius.plc",
        Txf, "txf", "vnd.mobius.txf",
        Mpn, "mpn", "vnd.mophun.application",
        Mpc, "mpc", "vnd.mophun.certificate",
        Xul, "xul", "vnd.mozilla.xul+xml",
        Cil, "cil", "vnd.ms-artgalry",
        Cab, "cab", "vnd.ms-cab-compressed",
        Xls, "xls", "vnd.ms-excel",
        Xlam, "xlam", "vnd.ms-excel.addin.macroenabled.12",
        Xlsb, "xlsb", "vnd.ms-excel.sheet.binary.macroenabled.12",
        Xlsm, "xlsm", "vnd.ms-excel.sheet.macroenabled.12",
        Xltm, "xltm", "vnd.ms-excel.template.macroenabled.12",
        Eot, "eot", "vnd.ms-fontobject",
        Chm, "chm", "vnd.ms-htmlhelp",
        Ims, "ims", "vnd.ms-ims",
        Lrm, "lrm", "vnd.ms-lrm",
        Thmx, "thmx", "vnd.ms-officetheme",
        Cat, "cat", "vnd.ms-pki.seccat",
        Stl, "stl", "vnd.ms-pki.stl",
        Ppt, "ppt", "vnd.ms-powerpoint",
        Ppam, "ppam", "vnd.ms-powerpoint.addin.macroenabled.12",
        Pptm, "pptm", "vnd.ms-powerpoint.presentation.macroenabled.12",
        Sldm, "sldm", "vnd.ms-powerpoint.slide.macroenabled.12",
        Ppsm, "ppsm", "vnd.ms-powerpoint.slideshow.macroenabled.12",
        Potm, "potm", "vnd.ms-powerpoint.template.macroenabled.12",
        Mpp, "mpp", "vnd.ms-project",
        Docm, "docm", "vnd.ms-word.document.macroenabled.12",
        Dotm, "dotm", "vnd.ms-word.template.macroenabled.12",
        Wps, "wps", "vnd.ms-works",
        Wpl, "wpl", "vnd.ms-wpl",
        Xps, "xps", "vnd.ms-xpsdocument",
        Mseq, "mseq", "vnd.mseq",
        Mus, "mus", "vnd.musician",
        Msty, "msty", "vnd.muvee.style",
        Taglet, "taglet", "vnd.mynfc",
        Nlu, "nlu", "vnd.neurolanguage.nlu",
        Ntf, "ntf", "vnd.nitf",
        Nnd, "nnd", "vnd.noblenet-directory",
        Nns, "nns", "vnd.noblenet-sealer",
        Nnw, "nnw", "vnd.noblenet-web",
        Ngdat, "ngdat", "vnd.nokia.n-gage.data",
        N_gage, "n-gage", "vnd.nokia.n-gage.symbian.install",
        Rpst, "rpst", "vnd.nokia.radio-preset",
        Rpss, "rpss", "vnd.nokia.radio-presets",
        Edm, "edm", "vnd.novadigm.edm",
        Edx, "edx", "vnd.novadigm.edx",
        Ext, "ext", "vnd.novadigm.ext",
        Odc, "odc", "vnd.oasis.opendocument.chart",
        Otc, "otc", "vnd.oasis.opendocument.chart-template",
        Odb, "odb", "vnd.oasis.opendocument.database",
        Odf, "odf", "vnd.oasis.opendocument.formula",
        Odft, "odft", "vnd.oasis.opendocument.formula-template",
        Odg, "odg", "vnd.oasis.opendocument.graphics",
        Otg, "otg", "vnd.oasis.opendocument.graphics-template",
        Odi, "odi", "vnd.oasis.opendocument.image",
        Oti, "oti", "vnd.oasis.opendocument.image-template",
        Odp, "odp", "vnd.oasis.opendocument.presentation",
        Otp, "otp", "vnd.oasis.opendocument.presentation-template",
        Ods, "ods", "vnd.oasis.opendocument.spreadsheet",
        Ots, "ots", "vnd.oasis.opendocument.spreadsheet-template",
        Odt, "odt", "vnd.oasis.opendocument.text",
        Odm, "odm", "vnd.oasis.opendocument.text-master",
        Ott, "ott", "vnd.oasis.opendocument.text-template",
        Oth, "oth", "vnd.oasis.opendocument.text-web",
        Xo, "xo", "vnd.olpc-sugar",
        Dd2, "dd2", "vnd.oma.dd2+xml",
        Oxt, "oxt", "vnd.openofficeorg.extension",
        Pptx, "pptx", "vnd.openxmlformats-officedocument.presentationml.presentation",
        Sldx, "sldx", "vnd.openxmlformats-officedocument.presentationml.slide",
        Ppsx, "ppsx", "vnd.openxmlformats-officedocument.presentationml.slideshow",
        Potx, "potx", "vnd.openxmlformats-officedocument.presentationml.template",
        Xlsx, "xlsx", "vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        Xltx, "xltx", "vnd.openxmlformats-officedocument.spreadsheetml.template",
        Docx, "docx", "vnd.openxmlformats-officedocument.wordprocessingml.document",
        Dotx, "dotx", "vnd.openxmlformats-officedocument.wordprocessingml.template",
        Mgp, "mgp", "vnd.osgeo.mapguide.package",
        Dp, "dp", "vnd.osgi.dp",
        Esa, "esa", "vnd.osgi.subsystem",
        Pdb, "pdb", "vnd.palm",
        Paw, "paw", "vnd.pawaafile",
        Str, "str", "vnd.pg.format",
        Ei6, "ei6", "vnd.pg.osasli",
        Efif, "efif", "vnd.picsel",
        Wg, "wg", "vnd.pmi.widget",
        Plf, "plf", "vnd.pocketlearn",
        Pbd, "pbd", "vnd.powerbuilder6",
        Box, "box", "vnd.previewsystems.box",
        Mgz, "mgz", "vnd.proteus.magazine",
        Qps, "qps", "vnd.publishare-delta-tree",
        Ptid, "ptid", "vnd.pvi.ptid1",
        Qxd, "qxd", "vnd.quark.quarkxpress",
        Bed, "bed", "vnd.realvnc.bed",
        Mxl, "mxl", "vnd.recordare.musicxml",
        Musicxml, "musicxml", "vnd.recordare.musicxml+xml",
        Cryptonote, "cryptonote", "vnd.rig.cryptonote",
        Cod, "cod", "vnd.rim.cod",
        Rm, "rm", "vnd.rn-realmedia",
        Rmvb, "rmvb", "vnd.rn-realmedia-vbr",
        Link66, "link66", "vnd.route66.link66+xml",
        St, "st", "vnd.sailingtracker.track",
        See, "see", "vnd.seemail",
        Sema, "sema", "vnd.sema",
        Semd, "semd", "vnd.semd",
        Semf, "semf", "vnd.semf",
        Ifm, "ifm", "vnd.shana.informed.formdata",
        Itp, "itp", "vnd.shana.informed.formtemplate",
        Iif, "iif", "vnd.shana.informed.interchange",
        Ipk, "ipk", "vnd.shana.informed.package",
        Twd, "twd", "vnd.simtech-mindmapper",
        Mmf, "mmf", "vnd.smaf",
        Teacher, "teacher", "vnd.smart.teacher",
        Sdkm, "sdkm", "vnd.solent.sdkm+xml",
        Dxp, "dxp", "vnd.spotfire.dxp",
        Sfs, "sfs", "vnd.spotfire.sfs",
        Sdc, "sdc", "vnd.stardivision.calc",
        Sda, "sda", "vnd.stardivision.draw",
        Sdd, "sdd", "vnd.stardivision.impress",
        Smf, "smf", "vnd.stardivision.math",
        Sdw, "sdw", "vnd.stardivision.writer",
        Sgl, "sgl", "vnd.stardivision.writer-global",
        Smzip, "smzip", "vnd.stepmania.package",
        Sm, "sm", "vnd.stepmania.stepchart",
        Sxc, "sxc", "vnd.sun.xml.calc",
        Stc, "stc", "vnd.sun.xml.calc.template",
        Sxd, "sxd", "vnd.sun.xml.draw",
        Std, "std", "vnd.sun.xml.draw.template",
        Sxi, "sxi", "vnd.sun.xml.impress",
        Sti, "sti", "vnd.sun.xml.impress.template",
        Sxm, "sxm", "vnd.sun.xml.math",
        Sxw, "sxw", "vnd.sun.xml.writer",
        Sxg, "sxg", "vnd.sun.xml.writer.global",
        Stw, "stw", "vnd.sun.xml.writer.template",
        Sus, "sus", "vnd.sus-calendar",
        Svd, "svd", "vnd.svd",
        Sis, "sis", "vnd.symbian.install",
        Xsm, "xsm", "vnd.syncml+xml",
        Bdm, "bdm", "vnd.syncml.dm+wbxml",
        Xdm, "xdm", "vnd.syncml.dm+xml",
        Tao, "tao", "vnd.tao.intent-module-archive",
        Pcap, "pcap", "vnd.tcpdump.pcap",
        Tmo, "tmo", "vnd.tmobile-livetv",
        Tpt, "tpt", "vnd.trid.tpt",
        Mxs, "mxs", "vnd.triscape.mxs",
        Tra, "tra", "vnd.trueapp",
        Ufd, "ufd", "vnd.ufdl",
        Utz, "utz", "vnd.uiq.theme",
        Umj, "umj", "vnd.umajin",
        Unityweb, "unityweb", "vnd.unity",
        Uoml, "uoml", "vnd.uoml+xml",
        Vcx, "vcx", "vnd.vcx",
        Vsd, "vsd", "vnd.visio",
        Vis, "vis", "vnd.visionary",
        Vsf, "vsf", "vnd.vsf",
        Wbxml, "wbxml", "vnd.wap.wbxml",
        Wmlc, "wmlc", "vnd.wap.wmlc",
        Wmlsc, "wmlsc", "vnd.wap.wmlscriptc",
        Wtb, "wtb", "vnd.webturbo",
        Nbp, "nbp", "vnd.wolfram.player",
        Wpd, "wpd", "vnd.wordperfect",
        Wqd, "wqd", "vnd.wqd",
        Stf, "stf", "vnd.wt.stf",
        Xar, "xar", "vnd.xara",
        Xfdl, "xfdl", "vnd.xfdl",
        Hvd, "hvd", "vnd.yamaha.hv-dic",
        Hvs, "hvs", "vnd.yamaha.hv-script",
        Hvp, "hvp", "vnd.yamaha.hv-voice",
        Osf, "osf", "vnd.yamaha.openscoreformat",
        Osfpvg, "osfpvg", "vnd.yamaha.openscoreformat.osfpvg+xml",
        Saf, "saf", "vnd.yamaha.smaf-audio",
        Spf, "spf", "vnd.yamaha.smaf-phrase",
        Cmp, "cmp", "vnd.yellowriver-custom-menu",
        Zir, "zir", "vnd.zul",
        Zaz, "zaz", "vnd.zzazz.deck+xml",
        Vxml, "vxml", "voicexml+xml",
        Wgt, "wgt", "widget",
        Hlp, "hlp", "winhlp",
        Wsdl, "wsdl", "wsdl+xml",
        Wspolicy, "wspolicy", "wspolicy+xml",
        Media7z, "7z", "x-7z-compressed",
        Abw, "abw", "x-abiword",
        Ace, "ace", "x-ace-compressed",
        Dmg, "dmg", "x-apple-diskimage",
        Aab, "aab", "x-authorware-bin",
        Aam, "aam", "x-authorware-map",
        Aas, "aas", "x-authorware-seg",
        Bcpio, "bcpio", "x-bcpio",
        Torrent, "torrent", "x-bittorrent",
        Blb, "blb", "x-blorb",
        Bz, "bz", "x-bzip",
        Bz2, "bz2", "x-bzip2",
        Cbr, "cbr", "x-cbr",
        Vcd, "vcd", "x-cdlink",
        Cfs, "cfs", "x-cfs-compressed",
        Chat, "chat", "x-chat",
        Pgn, "pgn", "x-chess-pgn",
        Nsc, "nsc", "x-conference",
        Cpio, "cpio", "x-cpio",
        Csh, "csh", "x-csh",
        Deb, "deb", "x-debian-package",
        Dgc, "dgc", "x-dgc-compressed",
        Dir, "dir", "x-director",
        Wad, "wad", "x-doom",
        Ncx, "ncx", "x-dtbncx+xml",
        Dtb, "dtb", "x-dtbook+xml",
        Res, "res", "x-dtbresource+xml",
        Dvi, "dvi", "x-dvi",
        Evy, "evy", "x-envoy",
        Eva, "eva", "x-eva",
        Bdf, "bdf", "x-font-bdf",
        Gsf, "gsf", "x-font-ghostscript",
        Psf, "psf", "x-font-linux-psf",
        Otf, "otf", "x-font-otf",
        Pcf, "pcf", "x-font-pcf",
        Snf, "snf", "x-font-snf",
        Ttf, "ttf", "x-font-ttf",
        Pfa, "pfa", "x-font-type1",
        Woff, "woff", "font-woff",
        Arc, "arc", "x-freearc",
        Spl, "spl", "x-futuresplash",
        Gca, "gca", "x-gca-compressed",
        Ulx, "ulx", "x-glulx",
        Gnumeric, "gnumeric", "x-gnumeric",
        Gramps, "gramps", "x-gramps-xml",
        Gtar, "gtar", "x-gtar",
        Hdf, "hdf", "x-hdf",
        Install, "install", "x-install-instructions",
        Iso, "iso", "x-iso9660-image",
        Jnlp, "jnlp", "x-java-jnlp-file",
        Latex, "latex", "x-latex",
        Lzh, "lzh", "x-lzh-compressed",
        Mie, "mie", "x-mie",
        Prc, "prc", "x-mobipocket-ebook",
        Application, "application", "x-ms-application",
        Lnk, "lnk", "x-ms-shortcut",
        Wmd, "wmd", "x-ms-wmd",
        Wmz, "wmz", "x-ms-wmz",
        Xbap, "xbap", "x-ms-xbap",
        Mdb, "mdb", "x-msaccess",
        Obd, "obd", "x-msbinder",
        Crd, "crd", "x-mscardfile",
        Clp, "clp", "x-msclip",
        Exe, "exe", "x-msdownload",
        Mvb, "mvb", "x-msmediaview",
        Wmf, "wmf", "x-msmetafile",
        Mny, "mny", "x-msmoney",
        Pub, "pub", "x-mspublisher",
        Scd, "scd", "x-msschedule",
        Trm, "trm", "x-msterminal",
        Wri, "wri", "x-mswrite",
        Nc, "nc", "x-netcdf",
        Nzb, "nzb", "x-nzb",
        P12, "p12", "x-pkcs12",
        P7b, "p7b", "x-pkcs7-certificates",
        P7r, "p7r", "x-pkcs7-certreqresp",
        Rar, "rar", "x-rar-compressed",
        Ris, "ris", "x-research-info-systems",
        Sh, "sh", "x-sh",
        Shar, "shar", "x-shar",
        Swf, "swf", "x-shockwave-flash",
        Xap, "xap", "x-silverlight-app",
        Sql, "sql", "x-sql",
        Sit, "sit", "x-stuffit",
        Sitx, "sitx", "x-stuffitx",
        Srt, "srt", "x-subrip",
        Sv4cpio, "sv4cpio", "x-sv4cpio",
        Sv4crc, "sv4crc", "x-sv4crc",
        T3, "t3", "x-t3vm-image",
        Gam, "gam", "x-tads",
        Tar, "tar", "x-tar",
        Tcl, "tcl", "x-tcl",
        Tex, "tex", "x-tex",
        Tfm, "tfm", "x-tex-tfm",
        Texinfo, "texinfo", "x-texinfo",
        Obj, "obj", "x-tgif",
        Ustar, "ustar", "x-ustar",
        Src, "src", "x-wais-source",
        Der, "der", "x-x509-ca-cert",
        Fig, "fig", "x-xfig",
        Xlf, "xlf", "x-xliff+xml",
        Xpi, "xpi", "x-xpinstall",
        Xz, "xz", "x-xz",
        Z1, "z1", "x-zmachine",
        Xaml, "xaml", "xaml+xml",
        Xdf, "xdf", "xcap-diff+xml",
        Xenc, "xenc", "xenc+xml",
        Xhtml, "xhtml", "xhtml+xml",
        Xml, "xml", "xml",
        Dtd, "dtd", "xml-dtd",
        Xop, "xop", "xop+xml",
        Xpl, "xpl", "xproc+xml",
        Xslt, "xslt", "xslt+xml",
        Xspf, "xspf", "xspf+xml",
        Mxml, "mxml", "xv+xml",
        Yang, "yang", "yang",
        Yin, "yin", "yin+xml",
        Zip, "zip", "zip",
        Wasm, "wasm", "wasm",

    }

    "audio" => {

        Adp, "adp", "adpcm",
        Au, "au", "basic",
        Mid, "mid", "midi",
        Mp4a, "mp4a", "mp4",
        Mpga, "mpga", "mpeg",
        Oga, "oga", "ogg",
        S3m, "s3m", "s3m",
        Sil, "sil", "silk",
        Uva, "uva", "vnd.dece.audio",
        Eol, "eol", "vnd.digital-winds",
        Dra, "dra", "vnd.dra",
        Dts, "dts", "vnd.dts",
        Dtshd, "dtshd", "vnd.dts.hd",
        Lvp, "lvp", "vnd.lucent.voice",
        Pya, "pya", "vnd.ms-playready.media.pya",
        Ecelp4800, "ecelp4800", "vnd.nuera.ecelp4800",
        Ecelp7470, "ecelp7470", "vnd.nuera.ecelp7470",
        Ecelp9600, "ecelp9600", "vnd.nuera.ecelp9600",
        Rip, "rip", "vnd.rip",
        Weba, "weba", "webm",
        Aac, "aac", "x-aac",
        Aif, "aif", "x-aiff",
        Caf, "caf", "x-caf",
        Flac, "flac", "x-flac",
        Mka, "mka", "x-matroska",
        M3u, "m3u", "x-mpegurl",
        Wax, "wax", "x-ms-wax",
        Wma, "wma", "x-ms-wma",
        Ram, "ram", "x-pn-realaudio",
        Rmp, "rmp", "x-pn-realaudio-plugin",
        Wav, "wav", "x-wav",
        Xm, "xm", "xm",

    }

    "chemical" => {

        Cdx, "cdx", "x-cdx",
        Cif, "cif", "x-cif",
        Cmdf, "cmdf", "x-cmdf",
        Cml, "cml", "x-cml",
        Csml, "csml", "x-csml",
        Xyz, "xyz", "x-xyz",

    }

    "image" => {

        Bmp, "bmp", "bmp",
        Cgm, "cgm", "cgm",
        G3, "g3", "g3fax",
        Gif, "gif", "gif",
        Ief, "ief", "ief",
        Jpeg, "jpeg", "jpeg",
        Ktx, "ktx", "ktx",
        Png, "png", "png",
        Btif, "btif", "prs.btif",
        Sgi, "sgi", "sgi",
        Svg, "svg", "svg+xml",
        Tiff, "tiff", "tiff",
        Psd, "psd", "vnd.adobe.photoshop",
        Uvi, "uvi", "vnd.dece.graphic",
        Sub, "sub", "vnd.dvb.subtitle",
        Djvu, "djvu", "vnd.djvu",
        Dwg, "dwg", "vnd.dwg",
        Dxf, "dxf", "vnd.dxf",
        Fbs, "fbs", "vnd.fastbidsheet",
        Fpx, "fpx", "vnd.fpx",
        Fst, "fst", "vnd.fst",
        Mmr, "mmr", "vnd.fujixerox.edmics-mmr",
        Rlc, "rlc", "vnd.fujixerox.edmics-rlc",
        Mdi, "mdi", "vnd.ms-modi",
        Wdp, "wdp", "vnd.ms-photo",
        Npx, "npx", "vnd.net-fpx",
        Wbmp, "wbmp", "vnd.wap.wbmp",
        Xif, "xif", "vnd.xiff",
        Webp, "webp", "webp",
        Media3ds, "3ds", "x-3ds",
        Ras, "ras", "x-cmu-raster",
        Cmx, "cmx", "x-cmx",
        Fh, "fh", "x-freehand",
        Ico, "ico", "x-icon",
        Sid, "sid", "x-mrsid-image",
        Pcx, "pcx", "x-pcx",
        Pic, "pic", "x-pict",
        Pnm, "pnm", "x-portable-anymap",
        Pbm, "pbm", "x-portable-bitmap",
        Pgm, "pgm", "x-portable-graymap",
        Ppm, "ppm", "x-portable-pixmap",
        Rgb, "rgb", "x-rgb",
        Tga, "tga", "x-tga",
        Xbm, "xbm", "x-xbitmap",
        Xpm, "xpm", "x-xpixmap",
        Xwd, "xwd", "x-xwindowdump",

    }

    "message" => {

        Eml, "eml", "rfc822",

    }

    "model" => {

        Igs, "igs", "iges",
        Msh, "msh", "mesh",
        Dae, "dae", "vnd.collada+xml",
        Dwf, "dwf", "vnd.dwf",
        Gdl, "gdl", "vnd.gdl",
        Gtw, "gtw", "vnd.gtw",
        Mts, "mts", "vnd.mts",
        Vtu, "vtu", "vnd.vtu",
        Wrl, "wrl", "vrml",
        X3db, "x3db", "x3d+binary",
        X3dv, "x3dv", "x3d+vrml",
        X3d, "x3d", "x3d+xml",

    }

    "text" => {

        Appcache, "appcache", "cache-manifest",
        Ics, "ics", "calendar",
        Css, "css", "css",
        Csv, "csv", "csv",
        Html, "html", "html",
        N3, "n3", "n3",
        Txt, "txt", "plain",
        Dsc, "dsc", "prs.lines.tag",
        Rtx, "rtx", "richtext",
        Sgml, "sgml", "sgml",
        Tsv, "tsv", "tab-separated-values",
        T, "t", "troff",
        Ttl, "ttl", "turtle",
        Uri, "uri", "uri-list",
        Vcard, "vcard", "vcard",
        Curl, "curl", "vnd.curl",
        Dcurl, "dcurl", "vnd.curl.dcurl",
        Scurl, "scurl", "vnd.curl.scurl",
        Mcurl, "mcurl", "vnd.curl.mcurl",
        Fly, "fly", "vnd.fly",
        Flx, "flx", "vnd.fmi.flexstor",
        Gv, "gv", "vnd.graphviz",
        Media3dml, "3dml", "vnd.in3d.3dml",
        Spot, "spot", "vnd.in3d.spot",
        Jad, "jad", "vnd.sun.j2me.app-descriptor",
        Wml, "wml", "vnd.wap.wml",
        Wmls, "wmls", "vnd.wap.wmlscript",
        S, "s", "x-asm",
        C, "c", "x-c",
        F, "f", "x-fortran",
        Java, "java", "x-java-source",
        Opml, "opml", "x-opml",
        P, "p", "x-pascal",
        Nfo, "nfo", "x-nfo",
        Etx, "etx", "x-setext",
        Sfv, "sfv", "x-sfv",
        Uu, "uu", "x-uuencode",
        Vcs, "vcs", "x-vcalendar",
        Vcf, "vcf", "x-vcard",

    }

    "video" => {

        Media3gp, "3gp", "3gpp",
        Media3g2, "3g2", "3gpp2",
        H261, "h261", "h261",
        H263, "h263", "h263",
        H264, "h264", "h264",
        Jpgv, "jpgv", "jpeg",
        Jpm, "jpm", "jpm",
        Mj2, "mj2", "mj2",
        Mp4, "mp4", "mp4",
        Mpeg, "mpeg", "mpeg",
        Ogv, "ogv", "ogg",
        Qt, "qt", "quicktime",
        Uvh, "uvh", "vnd.dece.hd",
        Uvm, "uvm", "vnd.dece.mobile",
        Uvp, "uvp", "vnd.dece.pd",
        Uvs, "uvs", "vnd.dece.sd",
        Uvv, "uvv", "vnd.dece.video",
        Dvb, "dvb", "vnd.dvb.file",
        Fvt, "fvt", "vnd.fvt",
        Mxu, "mxu", "vnd.mpegurl",
        Pyv, "pyv", "vnd.ms-playready.media.pyv",
        Uvu, "uvu", "vnd.uvvu.mp4",
        Viv, "viv", "vnd.vivo",
        Webm, "webm", "webm",
        F4v, "f4v", "x-f4v",
        Fli, "fli", "x-fli",
        Flv, "flv", "x-flv",
        M4v, "m4v", "x-m4v",
        Mkv, "mkv", "x-matroska",
        Mng, "mng", "x-mng",
        Asf, "asf", "x-ms-asf",
        Vob, "vob", "x-ms-vob",
        Wm, "wm", "x-ms-wm",
        Wmv, "wmv", "x-ms-wmv",
        Wmx, "wmx", "x-ms-wmx",
        Wvx, "wvx", "x-ms-wvx",
        Avi, "avi", "x-msvideo",
        Movie, "movie", "x-sgi-movie",
        Smv, "smv", "x-smv",

    }

    "x-conference" => {

        Ice, "ice", "x-cooltalk",

    }
);
