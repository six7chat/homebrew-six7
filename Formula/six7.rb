class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.60"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.60"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.60/six7-0.7.60-arm64-apple-darwin.tar.gz"
      sha256 "bd34323bae13a5125ae9f8217a560a14591cdafd6dcbc3cd9fe1f93811f97d11"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
