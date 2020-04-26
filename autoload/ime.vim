function! ime#interp() abort
  let b:chinese_mode = v:true

  while v:true 
    let ch = getchar()

    if ch >= char2nr('a') && ch <= char2nr('z')
      call ime#rpc#input_char(nr2char(ch))
    elseif ch == 128 " backspace
      call ime#rpc#backspace()
    elseif ch == char2nr(',') " previous page
      call ime#rpc#previous_page()
    elseif ch == char2nr('.') " next page
      call ime#rpc#next_page()
    elseif ch >= char2nr('1') && ch <= char2nr('9')
      call ime#rpc#confirm(ch - char2nr('0'))
    elseif ch == 27 " esc
      call ime#rpc#cancel()
      break
    endif
  endwhile
endfunction

function! ime#feed(chr) abort
  let ch = char2nr(a:chr)
  if ch >= char2nr('a') && ch <= char2nr('z')
    call ime#rpc#input_char(nr2char(ch))
  elseif ch == 128 " backspace
    call ime#rpc#backspace()
  elseif ch == char2nr(',') " previous page
    call ime#rpc#previous_page()
  elseif ch == char2nr('.') " next page
    call ime#rpc#next_page()
  elseif ch >= char2nr('1') && ch <= char2nr('9')
    call ime#rpc#confirm(ch - char2nr('0'))
  elseif ch == 27 " esc
    call ime#rpc#cancel()
    break
  endif
endfunction

