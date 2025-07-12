// utils/api.js

const BASE_URL = "http://127.0.0.1:8000";

export async function apiRequest(
  endpoint,
  method = "GET",
  body = null,
  token = null,
) {
  const headers = {
    "Content-Type": "application/json",
    Accept: "application/json",
  };

  if (token) {
    headers["Authorization"] = `Bearer ${token}`;
  }

  const config = { method, headers };

  if (body) config.body = JSON.stringify(body);

  const response = await fetch(`${BASE_URL}${endpoint}`, config);

  if (!response.ok) {
    const error = await response.text();
    throw new Error(error || "API request failed");
  }

  return await response.json();
}

// Get experience detail
export const getExperienceDetail = (id, token = null) =>
  apiRequest(`/experience/${id}`, "GET", null, token);

// Update experience by id
export const updateExperienceById = (id, data, token) =>
  apiRequest(`/experience/${id}`, "PATCH", data, token);

export const deleteExperienceById = (id, token) =>
  apiRequest(`/experience/${id}`, "DELETE", null, token);

export async function addExperience(payload, token) {
  return await apiRequest("/experience", "POST", payload, token);
}
