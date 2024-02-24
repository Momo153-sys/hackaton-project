import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Nft { 'id' : bigint, 'owner' : string }
export interface Task { 'tasks' : Array<string> }
export interface _SERVICE {
  'add_task' : ActorMethod<[string], string>,
  'display_data' : ActorMethod<[string], string>,
  'display_list' : ActorMethod<[], string>,
  'erase_task' : ActorMethod<[string], string>,
  'initialize_list' : ActorMethod<[], string>,
  'nft_transfer' : ActorMethod<[string], string>,
}
export declare const idlFactory: IDL.InterfaceFactory;
