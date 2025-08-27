export interface NotificationProps {
  identifier: string;
  title: string;
  type: "warning" | "info" | "success" | "error";
  message: string;
  isRead: boolean;
  createdAt: string;
}
