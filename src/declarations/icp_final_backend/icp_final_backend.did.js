export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'add_task' : IDL.Func([IDL.Text], [IDL.Text], []),
    'display_data' : IDL.Func([IDL.Text], [IDL.Text], []),
    'display_list' : IDL.Func([], [IDL.Text], []),
    'erase_task' : IDL.Func([IDL.Text], [IDL.Text], []),
    'initialize_list' : IDL.Func([], [IDL.Text], []),
    'nft_transfer' : IDL.Func([IDL.Text], [IDL.Text], []),
  });
};
export const init = ({ IDL }) => { return []; };
