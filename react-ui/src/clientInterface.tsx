// This is the interface between the backend rust app and the frontend.

import { CreateWidgetRequest, WidgetModifier, ApiAction } from "./types";
import { API_URL } from "./constants.tsx";

export const createWidget = async (request: CreateWidgetRequest) => {
  const response = await fetch(`${API_URL}/widgets`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(request),
  });
  return response;
};

export const getWidgets = async () => {
  const response = await fetch(`${API_URL}/widgets`);
  return response;
};

export const deleteWidget = async (id: string) => {
  const response = await fetch(`${API_URL}/widgets/${id}`, {
    method: "DELETE",
  });
  return response;
};

export const getWidgetModifiers = async (widgetId: string) => {
  const response = await fetch(`${API_URL}/widgets/${widgetId}/modifiers`);
  return response;
};

export const addWidgetModifier = async (
  widgetId: string,
  modifier: WidgetModifier
) => {
  const response = await fetch(`${API_URL}/widgets/${widgetId}/modifiers`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(modifier),
  });
  return response;
};

export const deleteWidgetModifier = async (
  widgetId: string,
  modifierId: string
) => {
  const response = await fetch(
    `${API_URL}/widgets/${widgetId}/modifiers/${modifierId}`,
    {
      method: "DELETE",
    }
  );
  return response;
};

export const updateWidgetBounds = async (
  widgetId: string,
  bounds: { x: number; y: number; width: number; height: number }
) => {
  const response = await fetch(`${API_URL}/widgets/${widgetId}`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      type: "updatewidgetbounds",
      content: {
        widget_id: widgetId,
        bounds: bounds,
      },
    } as ApiAction),
  });
  return response;
};

export const toggleWidgetVisibility = async (
  widgetId: string,
  visible: boolean
) => {
  const response = await fetch(`${API_URL}/widgets/${widgetId}`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      type: "togglewidgetvisibility",
      content: {
        widget_id: widgetId,
        visible: visible,
      },
    } as ApiAction),
  });
  return response;
};

export const maximizeWidget = async (widgetId: string) => {
  const response = await fetch(`${API_URL}/widgets/${widgetId}`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      type: "maximizewidget",
      content: {
        widget_id: widgetId,
      },
    } as ApiAction),
  });
  return response;
};

export const minimizeWidget = async (widgetId: string) => {
  const response = await fetch(`${API_URL}/widgets/${widgetId}`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      type: "minimizewidget",
      content: {
        widget_id: widgetId,
      },
    } as ApiAction),
  });
  return response;
};
