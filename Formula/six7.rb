class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.13"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.13"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.13/six7-0.7.13-arm64-apple-darwin.tar.gz"
      sha256 "d67caec19c4356ba4ce1b98396fa23dc98558c17735012b35dd63e291d5bb9ed"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
