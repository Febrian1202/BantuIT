<?php

namespace App\Enums;

enum RoleName: string
{
    case Admin = "admin";
    case Manager = "manager";
    case Technician = "technician";
    case Employee = "employee";
}
