let s:job_id = get(s:, "job_id", 0)

function! ime#rpc#init() abort
  if s:job_id == 0
    let id = jobstart([s:bin], { 'rpc': v:true })

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

  return call('rpcrequest', [s:job_id + method] + a:000)
endfunction

function! ime#rpc#start_context() abort
  if exists('b:__ime_context_id')
    if b:__ime_context_id != ''
      call s:cancel_context(b:__ime_context_id)
    endif
  endif

  let b:__ime_context_id = s:uuid()
  call ime#rpc#request('start_context', b:__ime_context_id)
endfunction

function! ime#rpc#input_char(ch) abort
  if !exists('b:__ime_context_id')
    echoerr 'should start context first'
    return []
  endif

  return ime#rpc#request('input_char', b:__ime_context_id, a:ch)
endfunction

function! ime#rpc#backspace() abort
  if !exists('b:__ime_context_id')
    echoerr 'should start context first'
    return []
  endif

  return ime#rpc#request('backspace', b:__ime_context_id)
endfunction

function! ime#rpc#cancel() abort
  if !exists('b:__ime_context_id')
    return
  endif

  call s:cancel_context(b:__ime_context_id)
endfunction

function! s:uuid() abort
  if executable('uuidgen')
    return system('uuidgen')[:-2]
  else
python << endpy
import vim
from uuid import uuid4
vim.command("let l:new_uuid = '%s'"% str(uuid4()))
endpy
    return l:new_uuid
  endif
endfunction

function! s:cancel_context(id) abort
  call ime#rpc#request('cancel', id)
  
  unlet b:__ime_context_id
endfunction
