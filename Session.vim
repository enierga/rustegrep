let SessionLoad = 1
if &cp | set nocp | endif
let s:cpo_save=&cpo
set cpo&vim
imap <Nul> <C-Space>
inoremap <expr> <Up> pumvisible() ? "\" : "\<Up>"
inoremap <expr> <S-Tab> pumvisible() ? "\" : "\<S-Tab>"
inoremap <expr> <Down> pumvisible() ? "\" : "\<Down>"
nmap  :wa:RustRun 
map  :NERDTreeToggle
nmap  :wa:RustTest!
map  :wa:mksession!:qa
xmap S <Plug>VSurround
nmap T :wa:RustTest
nnoremap \d :YcmShowDetailedDiagnostic
nmap cS <Plug>CSurround
nmap cs <Plug>Csurround
nmap ds <Plug>Dsurround
vmap gx <Plug>NetrwBrowseXVis
nmap gx <Plug>NetrwBrowseX
xmap gS <Plug>VgSurround
nmap ySS <Plug>YSsurround
nmap ySs <Plug>YSsurround
nmap yss <Plug>Yssurround
nmap yS <Plug>YSurround
nmap ys <Plug>Ysurround
vnoremap <silent> <Plug>NetrwBrowseXVis :call netrw#BrowseXVis()
nnoremap <silent> <Plug>NetrwBrowseX :call netrw#BrowseX(expand((exists("g:netrw_gx")? g:netrw_gx : '<cfile>')),netrw#CheckIfRemote())
nnoremap <silent> <Plug>SurroundRepeat .
nnoremap <silent> <Plug>GitGutterPreviewHunk :GitGutterPreviewHunk
nnoremap <silent> <Plug>GitGutterUndoHunk :GitGutterUndoHunk
nnoremap <silent> <Plug>GitGutterStageHunk :GitGutterStageHunk
nnoremap <silent> <expr> <Plug>GitGutterPrevHunk &diff ? '[c' : ":\execute v:count1 . 'GitGutterPrevHunk'\"
nnoremap <silent> <expr> <Plug>GitGutterNextHunk &diff ? ']c' : ":\execute v:count1 . 'GitGutterNextHunk'\"
xnoremap <silent> <Plug>GitGutterTextObjectOuterVisual :call gitgutter#hunk#text_object(0)
xnoremap <silent> <Plug>GitGutterTextObjectInnerVisual :call gitgutter#hunk#text_object(1)
onoremap <silent> <Plug>GitGutterTextObjectOuterPending :call gitgutter#hunk#text_object(0)
onoremap <silent> <Plug>GitGutterTextObjectInnerPending :call gitgutter#hunk#text_object(1)
map <Right> :echo "Use l to move right."
map <Left> :echo "Use h to move left."
map <Down> :echo "Use j to move down."
map <Up> :echo "Use k to move up."
imap S <Plug>ISurround
imap s <Plug>Isurround
inoremap <expr> 	 pumvisible() ? "\" : "\	"
imap  <Plug>Isurround
let &cpo=s:cpo_save
unlet s:cpo_save
set background=dark
set backspace=indent,eol,start
set completefunc=youcompleteme#CompleteFunc
set completeopt=preview,menuone
set cpoptions=aAceFsB
set encoding=utf-8
set errorformat=%-G,%-Gerror:\ aborting\ %.%#,%-Gerror:\ Could\ not\ compile\ %.%#,%Eerror:\ %m,%Eerror[E%n]:\ %m,%Wwarning:\ %m,%Inote:\ %m,%C\ %#-->\ %f:%l:%c,%E\ \ left:%m,%C\ right:%m\ %f:%l:%c,%Z,%f:%l:%c:\ %t%*[^:]:\ %m,%f:%l:%c:\ %*\\d:%*\\d\ %t%*[^:]:\ %m,%-G%f:%l\ %s,%-G%*[\ ]^,%-G%*[\ ]^%*[~],%-G%*[\ ]...,%-G%\\s%#Downloading%.%#,%-G%\\s%#Compiling%.%#,%-G%\\s%#Finished%.%#,%-G%\\s%#error:\ Could\ not\ compile\ %.%#,%-G%\\s%#To\ learn\ more\\,%.%#,%-Gnote:\ Run\ with\ `RUST_BACKTRACE=%.%#,%.%#panicked\ at\ \\'%m\\'\\,\ %f:%l:%c
set expandtab
set fileencodings=ucs-bom,utf-8,default,latin1
set helplang=en
set laststatus=2
set printoptions=paper:letter
set ruler
set runtimepath=~/.vim,~/.vim/bundle/Vundle.vim,~/.vim/bundle/vim-gitgutter,~/.vim/bundle/vim-surround,~/.vim/bundle/nerdtree,~/.vim/bundle/syntastic,~/.vim/bundle/rust.vim,~/.vim/bundle/YouCompleteMe,~/.vim/bundle/gruvbox,~/.vim/bundle/lightline.vim,/var/lib/vim/addons,/usr/share/vim/vimfiles,/usr/share/vim/vim80,/usr/share/vim/vimfiles/after,/var/lib/vim/addons/after,~/.vim/after,~/.vim/bundle/Vundle.vim,~/.vim/bundle/Vundle.vim/after,~/.vim/bundle/vim-gitgutter/after,~/.vim/bundle/vim-surround/after,~/.vim/bundle/nerdtree/after,~/.vim/bundle/syntastic/after,~/.vim/bundle/rust.vim/after,~/.vim/bundle/YouCompleteMe/after,~/.vim/bundle/gruvbox/after,~/.vim/bundle/lightline.vim/after
set scrolloff=15
set shiftwidth=4
set shortmess=filnxtToOc
set statusline=%#warningmsg#%{SyntasticStatuslineFlag()}%*
set suffixes=.bak,~,.swp,.o,.info,.aux,.log,.dvi,.bbl,.blg,.brf,.cb,.ind,.idx,.ilg,.inx,.out,.toc
set tabline=%!lightline#tabline()
set tabstop=4
let s:so_save = &so | let s:siso_save = &siso | set so=0 siso=0
let v:this_session=expand("<sfile>:p")
silent only
cd ~/ps05-thegrep-i-have-a-phase-1-ticket-up-for-grabs
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
set shortmess=aoO
badd +0 src/nfa.rs
argglobal
silent! argdel *
$argadd src/nfa.rs
edit src/nfa.rs
set splitbelow splitright
set nosplitbelow
set nosplitright
wincmd t
set winminheight=1 winheight=1 winminwidth=1 winwidth=1
argglobal
let s:cpo_save=&cpo
set cpo&vim
nmap <buffer> [c <Plug>GitGutterPrevHunk
onoremap <buffer> <silent> [[ :call rust#Jump('o', 'Back')
xnoremap <buffer> <silent> [[ :call rust#Jump('v', 'Back')
nnoremap <buffer> <silent> [[ :call rust#Jump('n', 'Back')
nmap <buffer> \hp <Plug>GitGutterPreviewHunk
nmap <buffer> \hu <Plug>GitGutterUndoHunk
nmap <buffer> \hs <Plug>GitGutterStageHunk
nmap <buffer> ]c <Plug>GitGutterNextHunk
onoremap <buffer> <silent> ]] :call rust#Jump('o', 'Forward')
xnoremap <buffer> <silent> ]] :call rust#Jump('v', 'Forward')
nnoremap <buffer> <silent> ]] :call rust#Jump('n', 'Forward')
xmap <buffer> ac <Plug>GitGutterTextObjectOuterVisual
omap <buffer> ac <Plug>GitGutterTextObjectOuterPending
xmap <buffer> ic <Plug>GitGutterTextObjectInnerVisual
omap <buffer> ic <Plug>GitGutterTextObjectInnerPending
let &cpo=s:cpo_save
unlet s:cpo_save
setlocal keymap=
setlocal noarabic
setlocal autoindent
setlocal backupcopy=
setlocal balloonexpr=
setlocal nobinary
setlocal nobreakindent
setlocal breakindentopt=
setlocal bufhidden=
setlocal buflisted
setlocal buftype=
setlocal cindent
setlocal cinkeys=0{,0},!^F,o,O,0[,0]
setlocal cinoptions=L0,(0,Ws,J1,j1,m1
setlocal cinwords=for,if,else,while,loop,impl,mod,unsafe,trait,struct,enum,fn,extern,macro
setlocal colorcolumn=
setlocal comments=s0:/*!,m:\ ,ex:*/,s1:/*,mb:*,ex:*/,:///,://!,://
setlocal commentstring=//%s
setlocal complete=.,w,b,u,t,i
setlocal concealcursor=
setlocal conceallevel=0
setlocal completefunc=youcompleteme#CompleteFunc
setlocal nocopyindent
setlocal cryptmethod=
setlocal nocursorbind
setlocal nocursorcolumn
setlocal nocursorline
setlocal define=
setlocal dictionary=
setlocal nodiff
setlocal equalprg=
setlocal errorformat=%-G,%-Gerror:\ aborting\ %.%#,%-Gerror:\ Could\ not\ compile\ %.%#,%Eerror:\ %m,%Eerror[E%n]:\ %m,%Wwarning:\ %m,%Inote:\ %m,%C\ %#-->\ %f:%l:%c,%E\ \ left:%m,%C\ right:%m\ %f:%l:%c,%Z,%f:%l:%c:\ %t%*[^:]:\ %m,%f:%l:%c:\ %*\\d:%*\\d\ %t%*[^:]:\ %m,%-G%f:%l\ %s,%-G%*[\ ]^,%-G%*[\ ]^%*[~],%-G%*[\ ]...,%-G%\\s%#Downloading%.%#,%-G%\\s%#Compiling%.%#,%-G%\\s%#Finished%.%#,%-G%\\s%#error:\ Could\ not\ compile\ %.%#,%-G%\\s%#To\ learn\ more\\,%.%#,%-Gnote:\ Run\ with\ `RUST_BACKTRACE=%.%#,%.%#panicked\ at\ \\'%m\\'\\,\ %f:%l:%c
setlocal expandtab
if &filetype != 'rust'
setlocal filetype=rust
endif
setlocal fixendofline
setlocal foldcolumn=0
setlocal foldenable
setlocal foldexpr=0
setlocal foldignore=#
setlocal foldlevel=0
setlocal foldmarker={{{,}}}
setlocal foldmethod=manual
setlocal foldminlines=1
setlocal foldnestmax=20
setlocal foldtext=foldtext()
setlocal formatexpr=
setlocal formatoptions=croqnlj
setlocal formatlistpat=^\\s*\\d\\+[\\]:.)}\\t\ ]\\s*
setlocal formatprg=
setlocal grepprg=
setlocal iminsert=0
setlocal imsearch=-1
setlocal include=
setlocal includeexpr=substitute(v:fname,'::','/','g')
setlocal indentexpr=GetRustIndent(v:lnum)
setlocal indentkeys=0{,0},!^F,o,O,0[,0]
setlocal noinfercase
setlocal iskeyword=@,48-57,_,192-255
setlocal keywordprg=
setlocal nolinebreak
setlocal nolisp
setlocal lispwords=
setlocal nolist
setlocal makeencoding=
setlocal makeprg=cargo\ $*
setlocal matchpairs=(:),{:},[:],<:>
setlocal modeline
setlocal modifiable
setlocal nrformats=bin,octal,hex
set number
setlocal number
setlocal numberwidth=4
setlocal omnifunc=
setlocal path=
setlocal nopreserveindent
setlocal nopreviewwindow
setlocal quoteescape=\\
setlocal noreadonly
setlocal norelativenumber
setlocal norightleft
setlocal rightleftcmd=search
setlocal noscrollbind
setlocal shiftwidth=4
setlocal noshortname
setlocal signcolumn=auto
setlocal smartindent
setlocal softtabstop=4
setlocal nospell
setlocal spellcapcheck=[.?!]\\_[\\])'\"\	\ ]\\+
setlocal spellfile=
setlocal spelllang=en
setlocal statusline=%{lightline#link()}%#LightlineLeft_active_0#%(\ %{lightline#mode()}\ %)%{(&paste)?\"|\":\"\"}%(\ %{&paste?\"PASTE\":\"\"}\ %)%#LightlineLeft_active_0_1#%#LightlineLeft_active_1#%(\ %R\ %)%{(&readonly)&&(1||(&modified||!&modifiable))?\"|\":\"\"}%(\ %t\ %)%{(&modified||!&modifiable)?\"|\":\"\"}%(\ %M\ %)%#LightlineLeft_active_1_2#%#LightlineMiddle_active#%=%#LightlineRight_active_2_3#%#LightlineRight_active_2#%(\ %{&ff}\ %)%{1||1?\"|\":\"\"}%(\ %{&fenc!=#\"\"?&fenc:&enc}\ %)%{1?\"|\":\"\"}%(\ %{&ft!=#\"\"?&ft:\"no\ ft\"}\ %)%#LightlineRight_active_1_2#%#LightlineRight_active_1#%(\ %3p%%\ %)%#LightlineRight_active_0_1#%#LightlineRight_active_0#%(\ %3l:%-2v\ %)
setlocal suffixesadd=.rs
setlocal swapfile
setlocal synmaxcol=3000
if &syntax != 'rust'
setlocal syntax=rust
endif
setlocal tabstop=8
setlocal tagcase=
setlocal tags=
setlocal termkey=
setlocal termsize=
setlocal textwidth=99
setlocal thesaurus=
setlocal noundofile
setlocal undolevels=-123456
setlocal nowinfixheight
setlocal nowinfixwidth
setlocal wrap
setlocal wrapmargin=0
silent! normal! zE
let s:l = 95 - ((41 * winheight(0) + 35) / 70)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
95
normal! 033|
lcd ~/ps05-thegrep-i-have-a-phase-1-ticket-up-for-grabs
tabnext 1
if exists('s:wipebuf')
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20 shortmess=filnxtToOc
set winminheight=1 winminwidth=1
let s:sx = expand("<sfile>:p:r")."x.vim"
if file_readable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &so = s:so_save | let &siso = s:siso_save
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
