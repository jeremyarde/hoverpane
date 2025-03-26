import { z } from "zod";

export const widgetSchema = z.object({
  title: z.string().optional(),
  widget_type: z.enum(["source", "url", "file"]),
  level: z.enum(["normal", "alwaysontop", "alwaysonbottom"]),
  url: z.string().optional(),
  selector: z.string().optional(),
  refresh_interval: z.number().optional(),
  html: z.string().optional(),
  position: z.tuple([z.number().min(0), z.number().min(0)]),
  size: z.tuple([z.number().min(0), z.number().min(0)]),
  background_color: z.tuple([
    z.number().min(0).max(255),
    z.number().min(0).max(255),
    z.number().min(0).max(255),
    z.number().refine((n) => n >= 0 && n <= 1, {
      message: "Alpha must be between 0 and 1",
    }),
  ]),
  resizeable: z.boolean().optional(),
  movable: z.boolean().optional(),
});

export type WidgetFormValues = z.infer<typeof widgetSchema>;
