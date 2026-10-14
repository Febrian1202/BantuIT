"use client";

import { useAuth } from "@/components/providers/auth-provider";
import { apiFetch } from "@/lib/client/api";
import { referenceKeys } from "@/lib/query-key";
import { DepartmentReference, TechnicianOption } from "@/types/auth";
import {
  TicketCategory,
  TicketPriority,
  TicketStatusReference,
} from "@/types/tickets";
import { useQuery } from "@tanstack/react-query";

const STALE_TIME = 5 * 60 * 1000; // 5 Menit

export function useReferenceData() {
  const { can } = useAuth();
  const canViewTechnicians = can("technician.list");

  const categoriesQuery = useQuery({
    queryKey: referenceKeys.ticketCategories(),
    queryFn: () => apiFetch<TicketCategory[]>("/ticket-categories"),
    staleTime: STALE_TIME,
  });

  const prioritiesQuery = useQuery({
    queryKey: referenceKeys.ticketPriorities(),
    queryFn: () => apiFetch<TicketPriority[]>("/ticket-priorities"),
    staleTime: STALE_TIME,
  });

  const statusesQuery = useQuery({
    queryKey: referenceKeys.ticketStatuses(),
    queryFn: () => apiFetch<TicketStatusReference[]>("/ticket-statuses"),
    staleTime: STALE_TIME,
  });

  const departmentsQuery = useQuery({
    queryKey: referenceKeys.departments(),
    queryFn: () => apiFetch<DepartmentReference[]>("/departments"),
    staleTime: STALE_TIME,
  });

  const techniciansQuery = useQuery({
    queryKey: referenceKeys.technicians(),
    queryFn: () => apiFetch<TechnicianOption[]>("/technicians"),
    staleTime: STALE_TIME,
  });

  return {
    categories: categoriesQuery.data?.data ?? [],
    priorities: prioritiesQuery.data?.data ?? [],
    statuses: statusesQuery.data?.data ?? [],
    departments: departmentsQuery.data?.data ?? [],
    technicians: techniciansQuery.data?.data ?? [],
    isLoading:
      categoriesQuery.isLoading ||
      prioritiesQuery.isLoading ||
      statusesQuery.isLoading ||
      departmentsQuery.isLoading ||
      (canViewTechnicians && techniciansQuery.isLoading),
  };
}
