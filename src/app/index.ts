
// import { hoge } from "../wasm/main.wasm"

const rust = import("../rust/wasm_rust");

rust.then(m => {
  // m.greet('world');
  // module.tutorial2();
  console.log('ADD', m.add(1,2));
  console.log('call get_name', m.get_name('HONDA'));
  console.log (['a', 'abcd', 'abcdefg'].filter(m.has_name_langth_file_over));
});
