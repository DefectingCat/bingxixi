import { fetch } from "@tauri-apps/plugin-http";
import { useAppState } from "../store";

const baseUrl = "https://mms.pinduoduo.com";

export interface MMSUser {
  id: number;
  username: string;
  mobile: string;
  nickname: string;
  mallOwner: boolean;
  roleId: number;
  isBindWeChat: boolean;
  forceMobileVerify: boolean;
  mallVerify: boolean;
  mallInfoAuthority: boolean;
  conjoinedUserId: null;
  mall_id: number;
  password_status: number;
  server_time: number;
  created_at: Date;
  updated_at: Date;
}
export async function userInfo() {
  const appState = useAppState();
  const cookie = appState.appState.mms.cookie;

  console.log("cookie", cookie);

  const res = await fetch(`${baseUrl}/janus/api/new/userinfo`, {
    method: "POST",
  });
  const response = (await res.json()) as MMSUser | null;
  return response;
}
