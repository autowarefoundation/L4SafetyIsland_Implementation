% =====================================================================
%  ntuslides.cls --- house beamer class for the
%  NTU / NEWS Lab / Autoware Foundation slide decks
%
%  Derived from ../Slides/preamble.tex, which was styled to the Keynote
%  reference template (../Slides/template.tex and its PDF).
%
%  Usage in the root file:
%
%      \documentclass{ntuslides}          % 16:9, 9pt, table already set
%
%      \title{...}
%      \subtitle{...}
%      \speaker{Prof.\ Chi-Sheng (Daniel) Shih}
%      \institute{National Taiwan University}
%      \event{Reference Design WG}
%      \date{2026-08-27}
%
%      \begin{document}
%      \titleframe
%      ...
%      \end{document}
%
%  The deck identity (title, speaker, event, date) lives in the root
%  file; this class only decides how it is rendered.
%
%  Class options
%      8pt 9pt 10pt 11pt 12pt 14pt 17pt 20pt   base size   (default 9pt)
%      aspectratio=<n>                          slide shape (default 169)
%      notiers                                  drop the criticality-tier
%                                               layer (see below)
%      any other option is forwarded to beamer (handout, notes=..., ...)
%
%  Assets (all optional, all replaceable from the root file):
%      \assetdir{<dir>/}          where the defaults below are looked up,
%                                 relative to the root .tex (default figures/)
%      \titlebackground{<file>}   full-bleed photo on the title slide
%      \titlelogoleft{<file>}     \titlelogoright{<file>}
%      \footerlogoleft{<file>}    \footerlogoright{<file>}
%      \sectionlogo{<file>}
%  Each takes an optional [<height>] first argument, e.g.
%      \footerlogoleft[4mm]{figures/logo-ntu-news.png}
%  A missing file is skipped with a warning rather than an error, so the
%  class compiles in a project that has no artwork yet.
%
%  Engine: pdflatex (xelatex / lualatex also work)
%
%  Build twice. The title and section slides position artwork against
%  `current page' with TikZ's remember-picture/overlay, which needs the
%  previous run's .aux to land correctly.
% =====================================================================

\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{ntuslides}[2026/08/28 v1.0 NTU/NEWS Lab beamer house class]

% ---------------------------------------------------------------------
%  Options
% ---------------------------------------------------------------------
\newif\ifntus@fontsize  \ntus@fontsizefalse
\newif\ifntus@aspect    \ntus@aspectfalse
\newif\ifntus@tiers     \ntus@tierstrue

\DeclareOption{tiers}{\ntus@tierstrue}
\DeclareOption{notiers}{\ntus@tiersfalse}

\DeclareOption{8pt} {\ntus@fontsizetrue\PassOptionsToClass{8pt}{beamer}}
\DeclareOption{9pt} {\ntus@fontsizetrue\PassOptionsToClass{9pt}{beamer}}
\DeclareOption{10pt}{\ntus@fontsizetrue\PassOptionsToClass{10pt}{beamer}}
\DeclareOption{11pt}{\ntus@fontsizetrue\PassOptionsToClass{11pt}{beamer}}
\DeclareOption{12pt}{\ntus@fontsizetrue\PassOptionsToClass{12pt}{beamer}}
\DeclareOption{14pt}{\ntus@fontsizetrue\PassOptionsToClass{14pt}{beamer}}
\DeclareOption{17pt}{\ntus@fontsizetrue\PassOptionsToClass{17pt}{beamer}}
\DeclareOption{20pt}{\ntus@fontsizetrue\PassOptionsToClass{20pt}{beamer}}

% aspectratio=<n> is key-value, so it cannot be \DeclareOption'd here;
% spot it in the fallback handler instead and suppress our own default.
\def\ntus@testaspect#1aspectratio#2\@nil{%
  \def\ntus@tempa{#2}%
  \ifx\ntus@tempa\@empty\else\ntus@aspecttrue\fi}
\DeclareOption*{%
  \expandafter\ntus@testaspect\CurrentOption aspectratio\@nil
  \PassOptionsToClass{\CurrentOption}{beamer}}

\ProcessOptions\relax

\ifntus@fontsize\else\PassOptionsToClass{9pt}{beamer}\fi
\ifntus@aspect  \else\PassOptionsToClass{aspectratio=169}{beamer}\fi
% Colour-aware tabular rows. beamer has no `table' option of its own -- in a
% plain \documentclass[table]{beamer} it reaches xcolor as a *global* option,
% which \PassOptionsToClass cannot reproduce, so hand it to xcolor directly.
\PassOptionsToPackage{table}{xcolor}
\LoadClass{beamer}

% =====================================================================
%  Packages
% =====================================================================
\RequirePackage{iftex}

\ifPDFTeX
  \RequirePackage[T1]{fontenc}
  \RequirePackage[utf8]{inputenc}
  % Fira is the house face. Fall back to the default sans rather than
  % failing, so the class works on a minimal TeX installation.
  \IfFileExists{FiraSans.sty}{%
    \RequirePackage[sfdefault,lf]{FiraSans}%
    \IfFileExists{FiraMono.sty}{\RequirePackage{FiraMono}}{}%
  }{%
    \ClassWarningNoLine{ntuslides}{Package `fira' not installed;
      falling back to the default sans face}%
    \renewcommand*\familydefault{\sfdefault}%
  }
\else
  \RequirePackage{fontspec}
  \IfFontExistsTF{Fira Sans}{%
    \setsansfont{Fira Sans}[Numbers=Lining]%
    \IfFontExistsTF{Fira Mono}{\setmonofont{Fira Mono}}{}%
  }{%
    \ClassWarningNoLine{ntuslides}{Fira Sans not found;
      falling back to the default sans face}%
  }
  \renewcommand*\familydefault{\sfdefault}
\fi

\RequirePackage{amsmath,amssymb}
\RequirePackage{booktabs}
\RequirePackage{multirow}
\RequirePackage{array}
\RequirePackage{tabularx}
\RequirePackage{graphicx}
\RequirePackage{adjustbox}
\RequirePackage{tikz}
\RequirePackage{xcolor}
\RequirePackage{appendixnumberbeamer}
\RequirePackage{microtype}
\RequirePackage{pifont}
\RequirePackage{underscore}   % printable _ in text mode: beamer re-tokenises
                              % frame bodies, so a \catcode trick cannot work

% Slides do not hyphenate: a word broken across a narrow table cell or
% callout reads badly. Done by switching to a language with no patterns
% loaded, rather than with hyphenat[none] -- that package and underscore
% both define \BreakableUnderscore and clash in either load order.
\newlanguage\nohyphenlanguage
\language=\nohyphenlanguage

\usetikzlibrary{arrows.meta,positioning,calc,fit,backgrounds,shapes.geometric,
                decorations.pathreplacing}

\graphicspath{{./}}

% =====================================================================
%  Colours --- house palette, sampled from the reference template
% =====================================================================
\definecolor{tblue}{HTML}{5E90E3}   % title blue
\definecolor{tgrey}{HTML}{A9A9A9}   % subtitle grey
\definecolor{tpanel}{HTML}{E8E8E8}  % section panel grey
\definecolor{tpage}{HTML}{C8C8C8}   % page-number grey

\definecolor{deep}{HTML}{2A5DA8}    % readable blue for body emphasis
\definecolor{teal}{HTML}{157A82}
\definecolor{amber}{HTML}{D98324}
\definecolor{brick}{HTML}{B5321F}
\definecolor{moss}{HTML}{2E7D4F}
\definecolor{plum}{HTML}{6B3FA0}
\definecolor{ink}{HTML}{141414}
\definecolor{mist}{HTML}{8A8A8A}
\definecolor{paper}{HTML}{FFFFFF}
\definecolor{rule}{HTML}{DCDCDC}

% =====================================================================
%  Deck identity --- all of it is set in the ROOT file
%
%  \title \subtitle \date \institute are beamer's own.
%  \speaker is an alias for \author; \event is added here.
% =====================================================================
\newcommand*{\speaker}[1]{\author{#1}}
% \def, not \let: \author redefines \insertauthor, so the alias has to
% resolve at use time rather than snapshot the (still empty) value here.
\newcommand*{\insertspeaker}{\insertauthor}

\newcommand*{\insertevent}{}
\newcommand*{\event}[1]{\renewcommand*{\insertevent}{#1}}

\institute{}
\date{}

% Test whether a piece of deck metadata was actually supplied.
\newcommand*{\ntus@ifempty}[3]{%
  \protected@edef\ntus@tempb{#1}%
  \ifx\ntus@tempb\@empty #2\else #3\fi}

% Institute / event / date, joined by bullets, skipping the empty ones.
% Emptiness is decided by typesetting into a box and measuring it, not by
% expanding the macro: beamer builds \insertinstitute out of \def's, which
% no \edef-based test can look inside.
\newif\ifntus@firstmeta
\newsavebox{\ntus@metabox}
\newcommand{\ntus@meta}[1]{%
  \sbox{\ntus@metabox}{#1}%
  \ifdim\wd\ntus@metabox>\z@
    \ifntus@firstmeta\global\ntus@firstmetafalse\else\;\textbullet\;\fi
    \usebox{\ntus@metabox}%
  \fi}
\newcommand{\ntus@metaline}{%
  \global\ntus@firstmetatrue
  \ntus@meta{\insertinstitute}\ntus@meta{\insertevent}\ntus@meta{\insertdate}}

% =====================================================================
%  Assets --- every one optional, every one overridable in the root file
% =====================================================================
% Warn once per file, not once per frame: the footer logos are drawn on
% every slide, and a missing one would otherwise fill the log.
\newcommand{\ntus@warnmissing}[1]{%
  \expandafter\ifx\csname ntus@warned@#1\endcsname\relax
    \expandafter\gdef\csname ntus@warned@#1\endcsname{}%
    \ClassWarningNoLine{ntuslides}{Asset `#1' not found -- omitted}%
  \fi}

% \ntus@graphic{<includegraphics options>}{<file>}
\newcommand{\ntus@graphic}[2]{%
  \ntus@ifempty{#2}{}{\IfFileExists{#2}{\includegraphics[#1]{#2}}%
                                      {\ntus@warnmissing{#2}}}}

% Where the DEFAULT artwork lives, relative to the root .tex file. Keep the
% trailing slash. A deck with a figures/ folder of its own needs nothing;
% one that keeps its pictures elsewhere says e.g. \assetdir{../art/}.
% Paths handed to the setters below are used verbatim and ignore this.
\newcommand*{\ntus@assetdir}{figures/}
\newcommand*{\assetdir}[1]{\renewcommand*{\ntus@assetdir}{#1}}

% Declare a settable asset: \ntus@asset{<name>}{<default height>}{<basename>}
% creates \<name>[<height>]{<file>}, \ntus@<name>file and \ntus@<name>ht.
\newcommand{\ntus@asset}[3]{%
  \expandafter\def\csname ntus@#1file\endcsname{\ntus@assetdir #3}%
  \expandafter\def\csname ntus@#1ht\endcsname{#2}%
  \expandafter\newcommand\csname #1\endcsname[2][#2]{%
    \expandafter\def\csname ntus@#1ht\endcsname{##1}%
    \expandafter\def\csname ntus@#1file\endcsname{##2}}%
  \expandafter\newcommand\csname ntus@put#1\endcsname{%
    \ntus@graphic{height=\csname ntus@#1ht\endcsname}%
                 {\csname ntus@#1file\endcsname}}}

\ntus@asset{titlelogoleft} {6.4mm}{logo-ntu-news-band.png}
\ntus@asset{titlelogoright}{9.5mm}{logo-autoware-band.png}
\ntus@asset{footerlogoleft} {3.9mm}{logo-ntu-news.png}
\ntus@asset{footerlogoright}{3.8mm}{logo-autoware.png}
\ntus@asset{sectionlogo}{9mm}{logo-autoware-band.png}

% Title-slide background photo, plus the flat colour used when there is
% none (the bottom-of-slide text is white, so it must stay dark).
\newcommand*{\ntus@titlebgfile}{\ntus@assetdir title-bg.png}
\newcommand*{\titlebackground}[1]{\renewcommand*{\ntus@titlebgfile}{#1}}
\colorlet{titlebg}{ink}

% =====================================================================
%  Theme
% =====================================================================
\usetheme{default}
\usecolortheme{default}
\setbeamertemplate{navigation symbols}{}
\setbeamercolor{background canvas}{bg=paper}
\setbeamercolor{normal text}{fg=ink}
\setbeamercolor{structure}{fg=deep}
\setbeamercolor{alerted text}{fg=brick}
\setbeamerfont{frametitle}{series=\bfseries}
\setbeamerfont{framesubtitle}{size=\small,series=\bfseries}
\setbeamercolor{frametitle}{fg=tblue}
\setbeamercolor{framesubtitle}{fg=tgrey}
\setbeamersize{text margin left=10mm,text margin right=10mm}

% triangle bullets, per template
\setbeamertemplate{itemize item}{\raisebox{0.05ex}{\small\color{tblue}$\blacktriangleright$}}
\setbeamertemplate{itemize subitem}{\raisebox{0.05ex}{\scriptsize\color{tgrey}$\blacktriangleright$}}
\setbeamertemplate{itemize subsubitem}{\raisebox{0.05ex}{\tiny\color{tgrey}$\blacktriangleright$}}
\setbeamercolor{itemize item}{fg=tblue}
\setbeamercolor{itemize subitem}{fg=tgrey}
\setbeamercolor{enumerate item}{fg=tblue}
\setlength{\leftmargini}{4.2mm}
\setlength{\leftmarginii}{4.0mm}

% ---- frame title: large bold blue, flush left, no rule
\setbeamertemplate{frametitle}{%
  \nointerlineskip%
  \vspace{2.0mm}%
  \begin{beamercolorbox}[wd=\paperwidth,leftskip=10mm,rightskip=10mm]{frametitle}%
    {\usebeamerfont{frametitle}\fontsize{16}{18.5}\selectfont\insertframetitle}%
    \ifx\insertframesubtitle\@empty\else%
      \\[0.5ex]{\usebeamerfont{framesubtitle}\color{tgrey}\insertframesubtitle}%
    \fi%
  \end{beamercolorbox}%
  \vspace{1.2mm}%
}

% ---- footer: left logo, page number centre, right logo
%  Inset slightly tighter than the 10mm body margin, per the template.
\newlength{\ntusfootmargin}\setlength{\ntusfootmargin}{9mm}

%  Three fixed boxes rather than two \hfill's. Glue would centre the
%  number between the two logos, and the left logo is over three times
%  the width of the right one, so the number came out ~44pt right of the
%  page centre. With the outer boxes the same width, the middle box is
%  centred on the page whatever the artwork measures.
\newlength{\ntusfootside}\setlength{\ntusfootside}{0.34\paperwidth}
\newlength{\ntusfootmid} \setlength{\ntusfootmid} {\dimexpr\paperwidth-2\ntusfootside\relax}

\newcommand{\deckfooter}{%
  \leavevmode%
  \hbox{%
    \begin{beamercolorbox}[wd=\ntusfootside,ht=5.4mm,dp=0.4mm,sep=0pt,left]{}%
      \hspace{\ntusfootmargin}%
      \raisebox{-0.6mm}{\ntus@putfooterlogoleft}%
    \end{beamercolorbox}%
    \begin{beamercolorbox}[wd=\ntusfootmid,ht=5.4mm,dp=0.4mm,sep=0pt,center]{}%
      {\scriptsize\color{tpage}\insertframenumber}%
    \end{beamercolorbox}%
    \begin{beamercolorbox}[wd=\ntusfootside,ht=5.4mm,dp=0.4mm,sep=0pt,right]{}%
      \raisebox{-0.6mm}{\ntus@putfooterlogoright}%
      \hspace{\ntusfootmargin}%
    \end{beamercolorbox}%
  }%
  \vskip0pt%
}
\setbeamertemplate{footline}{\deckfooter}

% ---- title slide: full-bleed photo, grey band, logos
\newcommand{\titleframe}{%
  \begin{frame}[plain,noframenumbering]
    \begin{tikzpicture}[remember picture,overlay]
      % background: photo if we have one, flat colour if we do not
      \IfFileExists{\ntus@titlebgfile}{%
        \node[anchor=north west,inner sep=0pt] at (current page.north west)
          {\includegraphics[width=\paperwidth,height=\paperheight]{\ntus@titlebgfile}};%
      }{%
        \ntus@warnmissing{\ntus@titlebgfile}%
        \fill[titlebg] (current page.north west) rectangle (current page.south east);%
      }
      \fill[tpanel] ($(current page.north west)+(0,-26mm)$)
        rectangle ($(current page.north east)+(0,-66mm)$);
      \node[anchor=north west,text width=138mm,align=left,inner sep=0pt]
        at ($(current page.north west)+(11mm,-31mm)$)
        {\hyphenpenalty=10000\exhyphenpenalty=10000\raggedright
         {\color{ink}\bfseries\fontsize{19}{22}\selectfont\inserttitle}\\[2.4mm]
         {\color{deep}\fontsize{9.5}{12.5}\selectfont\insertsubtitle}};
      \node[anchor=south west,inner sep=0pt]
        at ($(current page.north west)+(11mm,-63.5mm)$)
        {\ntus@puttitlelogoleft};
      \node[anchor=south east,inner sep=0pt]
        at ($(current page.north east)+(-11mm,-64mm)$)
        {\ntus@puttitlelogoright};
      \node[anchor=south,align=center,inner sep=0pt]
        at ($(current page.south)+(0,8mm)$)
        {{\color{white}\bfseries\small\insertspeaker}\\[1.4mm]
         {\color{white!80}\scriptsize\ntus@metaline}};
    \end{tikzpicture}
  \end{frame}}

% ---- section slides: split layout, one line of framing per section
\newcommand{\sectionframing}{}
\newcommand{\setsectionframing}[1]{\renewcommand{\sectionframing}{#1}}
\AtBeginSection[]{%
  \begin{frame}[plain,noframenumbering]
    \begin{tikzpicture}[remember picture,overlay]
      \fill[tpanel] ($(current page.north east)+(0,0)$)
        rectangle ($(current page.south east)+(-0.5\paperwidth,0)$);
      \node[anchor=west,text width=0.44\paperwidth,align=left]
        at ($(current page.west)+(9mm,-8mm)$)
        {\hyphenpenalty=10000\exhyphenpenalty=10000\tolerance=9999
         \raggedright\color{tgrey}\bfseries\insertsection};
      \node[anchor=center,text width=0.38\paperwidth,align=center]
        at ($(current page.center)+(0.25\paperwidth,6mm)$)
        {\hyphenpenalty=10000\exhyphenpenalty=10000\tolerance=9999
         \color{ink}\large\sectionframing};
      \node[anchor=north,inner sep=0pt]
        at ($(current page.center)+(0.25\paperwidth,-14mm)$)
        {\ntus@putsectionlogo};
    \end{tikzpicture}
  \end{frame}}

% =====================================================================
%  Macros
% =====================================================================

% ---- package / topic names -------------------------------------------
\newcommand{\pkg}[1]{{\ttfamily #1}}
\newcommand{\pkgs}[1]{{\ttfamily\scriptsize #1}}

% ---- eyebrow label ----------------------------------------------------
\newcommand{\eyebrow}[1]{{\footnotesize\bfseries\color{tblue}#1}}

% ---- takeaway strip ---------------------------------------------------
\newcommand{\takeaway}[1]{%
  \vfill
  \begin{tikzpicture}
    \node[fill=tblue!7,inner sep=2.4mm,
          text width=\dimexpr\textwidth-5mm\relax,align=left] (tbx)
         {\hyphenpenalty=10000\exhyphenpenalty=10000\raggedright \scriptsize\color{ink}\textbf{\color{tblue}Takeaway.}\ #1};
    \fill[tblue] ($(tbx.north west)+(0,0)$) rectangle ($(tbx.south west)+(0.9mm,0)$);
  \end{tikzpicture}}

% ---- callout: left accent rule + faint tint ---------------------------
\newcommand{\callout}[2][amber]{%
  \begin{center}
  \begin{tikzpicture}
    \node[fill=#1!5,inner sep=2.8mm,text width=0.90\textwidth,align=left]
         (cbx) {\hyphenpenalty=10000\exhyphenpenalty=10000\raggedright #2};
    \fill[#1] ($(cbx.north west)+(0,0)$) rectangle ($(cbx.south west)+(0.9mm,0)$);
  \end{tikzpicture}
  \end{center}}

% ---- titled callout: same language, with a heading -------------------
% (not \note -- beamer owns that for speaker notes)
\newcommand{\notebox}[3][tblue]{%
  \begin{center}
  \begin{tikzpicture}
    \node[fill=#1!6,inner sep=2.6mm,text width=0.94\textwidth,align=left]
         (nbx) {\hyphenpenalty=10000\exhyphenpenalty=10000\raggedright {\bfseries\color{#1}#2}\\[0.9mm]#3};
    \fill[#1] ($(nbx.north west)+(0,0)$) rectangle ($(nbx.south west)+(0.9mm,0)$);
  \end{tikzpicture}
  \end{center}}

% ---- pull-quote frame -------------------------------------------------
%  An ordinary frame, not a [plain] one. A plain frame suppresses the
%  footline, and re-inserting it in the body puts it wherever the text
%  flow ends -- about 56pt above where every other frame carries it.
%  Letting beamer place the footline keeps the footer on one line across
%  the deck, and dropping noframenumbering stops this slide from
%  reprinting the previous slide's number. It has no frame title, so no
%  title is drawn.
\newcommand{\pullquote}[2][]{%
  \begin{frame}
    \vfill
    \begin{center}
      {\color{tblue}\rule{16mm}{2.5pt}}\\[6mm]
      \begin{minipage}{0.84\paperwidth}
        \centering
        {\Large\color{ink} #2}\\[5mm]
        \ifx\relax#1\relax\else{\small\color{tgrey} #1}\fi
      \end{minipage}
    \end{center}
    \vfill
  \end{frame}}

\newcommand{\source}[1]{%
  \vfill{\raggedleft\scriptsize\color{tgrey} #1\par}}

% ---- two-column shorthand --------------------------------------------
\newenvironment{cols}[1][T]{\begin{columns}[#1,onlytextwidth]}{\end{columns}}
\newenvironment{col}[1]{\begin{column}{#1\textwidth}}{\end{column}}

% ---- inline highlights ------------------------------------------------
\newcommand{\num}[1]{{\bfseries\color{amber}#1}}
\newcommand{\good}[1]{{\bfseries\color{moss}#1}}
\newcommand{\bad}[1]{{\bfseries\color{brick}#1}}
\newcommand{\yes}{{\color{moss}\ding{51}}}
\newcommand{\no} {{\color{brick}\ding{55}}}
\newcommand{\arrow}{{\color{mist}$\rightarrow$}}

% ---- table conveniences ----------------------------------------------
\newcolumntype{L}[1]{>{\raggedright\arraybackslash}p{#1}}
\newcolumntype{C}[1]{>{\centering\arraybackslash}p{#1}}
\newcommand{\thd}[1]{\textcolor{mist}{\footnotesize\bfseries #1}}
% X columns: ragged right and top aligned. Justified X with hyphenation
% disabled stretches word spacing badly in a narrow cell.
\renewcommand\tabularxcolumn[1]{>{\raggedright\arraybackslash}p{#1}}
\renewcommand{\arraystretch}{1.06}
\setlength{\tabcolsep}{4pt}

% ---- generic identity chip -------------------------------------------
% \NoCaseChange is in the LaTeX kernel from 2022-06; older TeX Live (e.g. 2021) needs this fallback.
\providecommand{\NoCaseChange}[1]{#1}
\newcommand{\chip}[2]{\NoCaseChange{%
  \tikz[baseline=(X.base)]{%
    \node[inner xsep=2.4pt,inner ysep=1.2pt,rounded corners=1pt,
          fill=#1!8,draw=#1,line width=0.4pt,text=#1](X)
         {\ttfamily\scriptsize\bfseries #2};}}}

% =====================================================================
%  Criticality-tier layer  (class option `notiers' switches it off)
%
%  These are not decoration: they encode the ASIL tier an element
%  belongs to, and are used consistently across the slides, the design
%  document and the reference diagrams. Each maps onto a house semantic
%  colour, so the coding survives inside the template palette.
% =====================================================================
\ifntus@tiers

\colorlet{siT0}{brick}   % T0  safety core         ASIL-D
\colorlet{siT1}{amber}   % T1  supervision         ASIL-B
\colorlet{siT1s}{plum}   % T1s safety sensing      ASIL-B
\colorlet{siT2}{deep}    % T2  nominal control     QM, monitored
\colorlet{siT3}{mist}    % T3  non-real-time       QM
\colorlet{siHPC}{moss}   % HPC domain

% row-fill tints of the same six, for \rowcolor
\colorlet{siT0bg}{siT0!8}   \colorlet{siT1bg}{siT1!10}
\colorlet{siT1sbg}{siT1s!8} \colorlet{siT2bg}{siT2!8}
\colorlet{siT3bg}{siT3!12}  \colorlet{siHPCbg}{siHPC!8}
\colorlet{siSigbg}{tblue!8}
\colorlet{siSignal}{deep}   % accent used by the architecture figures

% ---- node identity chips, coloured by criticality tier ----------------
\newcommand{\nzero}[1]{\chip{siT0}{#1}}
\newcommand{\none}[1] {\chip{siT1}{#1}}
\newcommand{\nsen}[1] {\chip{siT1s}{#1}}
\newcommand{\ntwo}[1] {\chip{siT2}{#1}}
\newcommand{\nthree}[1]{\chip{siT3}{#1}}
\newcommand{\nhpc}[1] {\chip{siHPC}{#1}}

% ---- tier legend, usable as a running key ----------------------------
\newcommand{\tierkey}{%
  \begin{center}\scriptsize
  \nzero{T0} safety core \quad \none{T1} supervision \quad
  \nsen{T1s} safety sensing \quad \ntwo{T2} nominal control \quad
  \nthree{T3} non-RT \quad \nhpc{HPC} HPC domain
  \end{center}}

% ---- TikZ block styles shared by the architecture figures ------------
\tikzset{
  dom/.style     ={rounded corners=2pt,draw,line width=0.9pt,inner sep=5pt,
                   align=center,font=\scriptsize},
  hpcbox/.style  ={dom,fill=siHPC!7, draw=siHPC},
  t0box/.style   ={dom,fill=siT0!7,  draw=siT0},
  t1box/.style   ={dom,fill=siT1!8,  draw=siT1},
  tsbox/.style   ={dom,fill=siT1s!7, draw=siT1s},
  t2box/.style   ={dom,fill=siT2!7,  draw=siT2},
  t3box/.style   ={dom,fill=siT3!12, draw=siT3},
  extbox/.style  ={dom,fill=black!3, draw=black!35},
  netbox/.style  ={dom,fill=tblue!8, draw=tblue},
  lbl/.style     ={font=\tiny\itshape,color=mist,align=center},
  flow/.style    ={-{Stealth[length=4pt]},line width=0.8pt,color=mist},
  fat/.style     ={-{Stealth[length=5pt]},line width=1.6pt,color=deep},
  dashflow/.style={-{Stealth[length=4pt]},line width=0.8pt,dashed,color=mist},
}

\fi

\endinput
