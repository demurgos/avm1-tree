import { CaseStyle } from "kryo/case-style";
import { DocumentIoType, DocumentType } from "kryo/types/document";
import { LiteralType } from "kryo/types/literal";
import { ActionBase } from "../action-base";
import { $ActionType, ActionType } from "../action-type";

export interface BitOr extends ActionBase {
  action: ActionType.BitOr;
}

export const $BitOr: DocumentIoType<BitOr> = new DocumentType<BitOr>({
  properties: {
    action: {type: new LiteralType({type: $ActionType, value: ActionType.BitOr as ActionType.BitOr})},
  },
  changeCase: CaseStyle.SnakeCase,
});
