function! ime#interp() abort
  let b:chinese_mode = v:true
  let b:candidate_selector = {
        \   'candidates': [],
        \   'page': 0,
        \   'codes': [],
        \ }

  while v:true 
    let ch = getchar()

    if ch >= char2nr('a') && ch <= char2nr('z')
      if !exists('b:__ime_context_id')
        call ime#rpc#start_context()
      endif

      " simple chars, these should be send to IMEngine
      let candidates = ime#rpc#input_char(nr2char(ch))

      call s:update_candidates(candidates)
      call s:render_candidates()
    elseif ch == 128 " backspace
      let candidates = ime#rpc#backspace()

      call s:update_candidates(candidates)
      call s:render_candidates()
    elseif ch == 27 " esc
      call ime#rpc#cancel()

      break
    endif
  endwhile

endfunction


function! s:update_candidates(candidates) abort
  let b:candidate_selector['codes'] = candidates.get('char_sets', [])
  let b:candidate_selector['page'] = 0
  let b:candidate_selector['candidates'] = candidates.get('candidates', [])
endfunction

function! s:render_candidates() abort
  if !exists('b:__ime_candidates_win')
  endif

  let width = len(b:candidate_selector.codes) - 1 +
        \ s:sum(map(b:candidate_selector.codes, 'len(v:val)'))
  let width = max(width, s:sum(map(
        \ b:candidate_selector.candidates, 
        \ 'len(v:val.text) / 3 * 2 + len(v:val.remain)')) +
        \ len(b:candidate_selector.candidates) * 2)

  let opt = {
        \ 'relative': 'cursor',
        \ 'row': 0,
        \ 'col': 0,
        \ 'width': width,
        \ 'height': 3,
        \ }
endfunction


function! s:sum(l) abort
  let res = 0
  for x in l
    let res += x
  endfor
  return res
endfunction

