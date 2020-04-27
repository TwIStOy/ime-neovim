function! ime#toggle() abort
  if exists('b:__ime_enable') && b:__ime_enable
    call s:disable_ime()
  else
    call s:enable_ime()
  endif
endfunction

function! s:enable_ime() abort
  let b:__ime_enable = v:true

  call ime#rpc#register()
endfunction

function! s:disable_ime() abort
  let b:__ime_enable = v:false

  call ime#rpc#unregister()
endfunction
