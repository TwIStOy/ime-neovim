let s:job_id = get(s:, "job_id", 0)

function! ime#rpc#init() abort
  if s:job_id == 0
    let id = jobstart([g:ime_bin], { 'rpc': v:true })

    if id <= 0
      echoerr 'start ime engine failed...'
    else
      let s:job_id = id
    endif
  else
    return s:job_id
  endif
endfunction

function! ime#rpc#request(method, ...) abort
  if s:job_id == 0 
    call ime#rpc#init()
  endif

  if s:job_id == 0
    return
  endif

  return call('rpcrequest', [s:job_id, a:method] + a:000)
endfunction

function! ime#rpc#register() abort
  call ime#rpc#request('register_events')
endfunction

function! ime#rpc#unregister() abort
  call ime#rpc#request('unregister_events')
endfunction

function! ime#rpc#start_context() abort
  if exists('b:__ime_context_id')
    if b:__ime_context_id != ''
      call s:cancel_context(b:__ime_context_id)
    endif
  endif

  let b:__ime_context_id = ime#rpc#request('start_context')
endfunction

function! ime#rpc#input_char(ch) abort
  if !exists('b:__ime_context_id')
    call ime#rpc#start_context()
  endif

  return ime#rpc#request('input_char', b:__ime_context_id, a:ch, bufnr('%'))
endfunction

function! ime#rpc#backspace() abort
  if !exists('b:__ime_context_id')
    call feedkeys("\<Bs>", 'n')
    return ""
  endif

  let res = ime#rpc#request('backspace', b:__ime_context_id, bufnr('%'))
  if res == "canceled"
    unlet b:__ime_context_id
  endif

  return ""
endfunction

function! ime#rpc#next_page() abort
  if !exists('b:__ime_context_id')
    return '.'
  endif

  call ime#rpc#request('next_page', bufnr('%'))
  return ""
endfunction

function! ime#rpc#previous_page() abort
  if !exists('b:__ime_context_id')
    return ','
  endif

  call ime#rpc#request('previous_page', bufnr('%'))
  return ""
endfunction

function! ime#rpc#feed_space() abort
  if !exists('b:__ime_context_id')
    return ' '
  endif

  return ime#rpc#confirm(1)
endfunction

function! ime#rpc#feed_number(num) abort
  if !exists('b:__ime_context_id')
    return string(a:num)
  endif

  return ime#rpc#confirm(a:num)
endfunction

function! ime#rpc#confirm(idx) abort
  if !exists('b:__ime_context_id')
    echoerr 'Should start context_id first.'
    return ''
  endif

  let txt = ime#rpc#request('confirm', b:__ime_context_id, a:idx, bufnr('%'))

  call ime#rpc#cancel()
  return txt
endfunction

function! ime#rpc#cancel() abort
  if !exists('b:__ime_context_id')
    call ime#rpc#start_context()
  endif

  call s:cancel_context(b:__ime_context_id)
endfunction

function! s:cancel_context(id) abort
  call ime#rpc#request('cancel', a:id, bufnr('%'))
  
  unlet b:__ime_context_id
endfunction
