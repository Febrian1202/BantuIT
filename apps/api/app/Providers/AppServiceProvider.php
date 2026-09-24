<?php

namespace App\Providers;

use App\Authorization\AbilityMatrix;
use App\Enums\RoleName;
use App\Models\Notification;
use App\Models\User;
use App\Policies\NotificationPolicy;
use Illuminate\Cache\RateLimiting\Limit;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Gate;
use Illuminate\Support\Facades\RateLimiter;
use Illuminate\Support\ServiceProvider;

class AppServiceProvider extends ServiceProvider
{
    /**
     * Register any application services.
     */
    public function register(): void
    {
        //
    }

    /**
     * Bootstrap any application services.
     */
    public function boot(): void
    {
        $this->configureRateLimiter();
        $this->registerGates();
    }

    /**
     * Konfigurasi rate limiter
     */
    private function configureRateLimiter(): void
    {
        RateLimiter::for("login", function (Request $request) {
            return app()->environment("testing", "local")
                ? Limit::none()
                : Limit::perMinute(5)->by($request->ip());
        });

        RateLimiter::for("upload", function (Request $request) {
            return Limit::perMinute(20)->by(
                $request->user()?->id ?: $request->ip(),
            );
        });

        RateLimiter::for("search", function (Request $request) {
            return Limit::perMinute(60)->by(
                $request->user()?->id ?: $request->ip(),
            );
        });

        RateLimiter::for("export", function (Request $request) {
            return Limit::perMinute(10)->by(
                $request->user()?->id ?: $request->ip(),
            );
        });

        RateLimiter::for("api", function (Request $request) {
            return Limit::perMinute(120)->by(
                $request->user()?->id ?: $request->ip(),
            );
        });
    }

    /**
     * Mendaftarkan Gates dari AbilityMatrix
     * plus pengecualian untuk admin
     */
    private function registerGates(): void
    {
        // Admin bypass, semua gate diizinkan untuk admin
        Gate::before(function (User $user, string $ability): ?bool {
            if (!$user->isAdmin()) {
                return null;
            }

            return in_array(
                $ability,
                AbilityMatrix::adminGateExceptions(),
                true,
            )
                ? null
                : true;
        });

        // RBAC dari matrix
        foreach (AbilityMatrix::getRoleAbilities() as $ability => $roles) {
            if (in_array($ability, ["user.deactivate", "user.delete"], true)) {
                continue;
            }

            Gate::define($ability, fn(User $user) => $user->hasRole(...$roles));
        }

        // Self-protection: admin tidak boleh menonaktifkan atau menghapus akunnya sendiri
        Gate::define("user.deactivate", function (
            User $user,
            ?User $target = null,
        ) {
            return $user->hasRole(RoleName::Admin) &&
                (!$target || $target->getKey() !== $user->getKey());
        });

        Gate::define("user.delete", function (
            User $user,
            ?User $target = null,
        ) {
            return $user->hasRole(RoleName::Admin) &&
                (!$target || $target->getKey() !== $user->getKey());
        });

        // Isolasi notifikasi: cek kepemilikan via NotificationPolicy
        $notificationPolicy = new NotificationPolicy();
        Gate::define("notification.viewAny", function (User $user) use (
            $notificationPolicy,
        ) {
            return $notificationPolicy->viewAny($user);
        });
        Gate::define("notification.markAsRead", function (
            User $user,
            Notification $notification,
        ) use ($notificationPolicy) {
            return $notificationPolicy->markAsRead($user, $notification);
        });
        Gate::define("notification.markAllAsRead", function (User $user) use (
            $notificationPolicy,
        ) {
            return $notificationPolicy->markAllAsRead($user);
        });
    }
}
