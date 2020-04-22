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
    echerr 'fuck...'
  elseif id == -1
  echoerr 'fuck...'
  else
    let s:ime_job_id = id
  endif
endfunction

function! IMEStartContext() abort
  call rpcrequest(s:ime_job_id, 'start_context', 'abcd')
endfunction

call s:connect()
