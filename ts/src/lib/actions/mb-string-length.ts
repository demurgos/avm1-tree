import { CaseStyle } from "kryo/case-style";
import { DocumentIoType, DocumentType } from "kryo/types/document";
import { LiteralType } from "kryo/types/literal";
import { ActionBase } from "../action-base";
import { $ActionType, ActionType } from "../action-type";

export interface MbStringLength extends ActionBase {
  action: ActionType.MbStringLength;
}

export const $MbStringLength: DocumentIoType<MbStringLength> = new DocumentType<MbStringLength>({
  properties: {
    action: {type: new LiteralType({type: $ActionType, value: ActionType.MbStringLength as ActionType.MbStringLength})},
  },
  changeCase: CaseStyle.SnakeCase,
});
