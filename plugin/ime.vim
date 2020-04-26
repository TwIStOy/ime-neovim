" Initialize the channel
if !exists('s:ime_job_id')
	let s:ime_job_id = 0
endif

" The path to the binary that was created out of 'cargo build' or 'cargo build --release". This will generally be 'target/release/name'
let s:bin = '/Users/twistoy/vim_plugin/ime-neovim/target/debug/ime'

function! s:init_rpc() abort
  if s:ime_job_id == 0
    let jid = jobstart([s:bin], { 'rpc': v:true })
    return jid
  else
    return s:ime_job_id
  endif
endfunction

function! s:connect() abort
  let id = s:init_rpc()

  if id == 0
    echoerr 'fuck...'
  elseif id == -1
  echoerr 'fuck...'
  else
    let s:ime_job_id = id
  endif
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

let s:context_id = get(s:, 'context_id', '')

function! IMEStartContext() abort
  if s:context_id != ''
    call IMECancel()
  endif

  let s:context_id = rpcrequest(s:ime_job_id, 'start_context')
  return s:context_id
endfunction

function! IMEInput(ch) abort
  if s:context_id == ''
    echo 'should start first'
    return ''
  endif

  let candidates = rpcrequest(s:ime_job_id, 'input_char',
        \ s:context_id, a:ch, bufnr('%'))
  echo candidates
endfunction

function! IMEBackspace() abort
  if s:context_id == ''
    echo 'should start first'
    return ''
  endif

  let candidates = rpcrequest(s:ime_job_id, 'backspace', s:context_id)
  echo candidates
endfunction

function! IMECancel(ch) abort
  if s:context_id == ''
    echo 'should start first'
    return ''
  endif

  call rpcrequest(s:ime_job_id, 'cancel', s:context_id)
  let s:context_id = ''
endfunction

call s:connect()
