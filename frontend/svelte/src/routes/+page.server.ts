import { app } from "$lib";
import { PaginationRequest } from "@breadcrumbs";
import type { PageLoad, Actions } from "$types";

// SPA
// export const ssr: boolean = false;
// export const prerender: boolean = false;

export const load: PageLoad = async () => {
  return {
    tasksPaginationResponse: (await app.viewTasks({
      paginationRequest: PaginationRequest.unbounded(),
    })).paginationResponse,
  };
};

export const actions = {
  create: async ({ request }) => {
    const formData = await request.formData();
    await app.createTask({
      taskDescription: formData.get("task-description"),
    });
  },
  delete: async ({ request }) => {
    const formData = await request.formData();
    await app.removeTask({
      taskId: formData.get("task-id"),
    });
  },
} satisfies Actions
